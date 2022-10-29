use std::fs::File;
use std::io::{copy};
use std::io::Cursor;
use crate::{env_or};
use tempfile::Builder;
use std::time::Duration;
use reqwest::{ClientBuilder, Proxy, Url};

pub const PROXY_ENV: &str = "PROTOC_HTTP_PROXY";


#[allow(unused)]
pub async fn build_download(url:&str) ->Result<reqwest::Response, reqwest::Error> {
    new_client_builder().build()?.get(url).send().await
}

#[allow(unused)]
pub fn new_client_builder() ->ClientBuilder {
    let v=  env_or!(PROXY_ENV.to_string()) ;
    println!("env: {}",v);
    if v!="" {
        reqwest::Client::builder().
            timeout(Duration::from_secs(3600)).
            proxy(Proxy::all(v).unwrap())
    }else {
        reqwest::Client::builder().timeout(Duration::from_secs(3600))
    }
}

#[allow(unused)]
pub async fn download_file(target:&str,path:&str)->u64 {

    let tmp_dir = Builder::new().prefix(path).tempdir().expect("tmp_dir error: ");
    let mut response = reqwest::get(target).await.expect("response get error: ");

    let mut dest = {
        let fs_name = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: {} ", fs_name);
        let fs_name = tmp_dir.path().join(fs_name);
        println!("will be located under: {:?} ", fs_name);
        File::create(fs_name).expect("create file error: ")
    };
    let content =  response.text().await.expect("file expect ");
    match copy(&mut content.as_bytes(), &mut dest) {
        Ok(u) => {
            u
        },
      _  => {
          0
       }
    }
}

type ResultRes<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[allow(unused)]
async fn fetch_file(url: String) -> ResultRes<String> {
    let response = build_download(url.clone().as_str()).await?;
    let file_url = Url::parse(url.as_str())?;
    let file_name = file_url.path_segments().
        and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");
    let mut file = std::fs::File::create(file_name.clone())?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(file_name.to_string())
}


#[allow(unused)]
#[cfg(test)]
mod test {
    use std::{fs};
    use std::fs::File;
    use crate::env_autoload;
    use std::io::{Read, Write};
    use crate::download::download::{build_download, download_file, fetch_file, PROXY_ENV};

    #[warn(unused_imports)]
    #[tokio::test]
    async fn test_download_text() {
        let url = "https://doc.rust-lang.org/std/any/index.html";
        let x = build_download(url).await;
        let resp = match x {
            Ok(resp) => {
                println!("status {}, {}",resp.status(),resp.content_length().unwrap());
                assert_eq!(resp.status(),reqwest::StatusCode::OK);
                resp
            }
            Err(e) => {
              unreachable!()
            }
        };

        let content = resp.
            text_with_charset("utf-8").
            await.
            expect("异常");
        println!("{}",content)

    }

    #[warn(unused_imports)]
    #[tokio::test]
    async fn test_download_file() {
        let url = "https://github.com/protocolbuffers/protobuf/releases/download/v21.5/protoc-21.5-win64.zip";
        let result = download_file(url,".").await;
        assert!(result>0,"下载失败")
    }

    #[warn(unused_imports)]
    #[tokio::test]
    async fn test_fetch_file() {
        // export https_proxy=http://127.0.0.1:7890 http_proxy=http://127.0.0.1:7890 all_proxy=socks5://127.0.0.1:7890
        env_autoload!();
        let url = "https://github.com/protocolbuffers/protobuf/releases/download/v21.5/protoc-21.5-win64.zip";
        let result = fetch_file(url.to_string()).await;
        match result {
            Ok(file) => {
                 assert_ne!(file,"","下载失败");
                 println!("下载成功 {}",file);
                 fs::remove_file(file);
            },
            _ => {
               println!("下载失败")
            }
        }
    }

}