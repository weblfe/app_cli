use std::{io};
use std::collections::HashMap;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout,Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

#[derive(Clone)]
pub struct TableRender<'a>  {
    state:  TableState,
    columns: Vec<Vec<&'a str>>,
    header :  Vec<&'a str>,
    title : &'a str,
}

#[allow(unused)]
impl <'a> TableRender<'a>  {

    pub fn new() -> TableRender<'a> {
        TableRender {
            state:TableState::default(),
            columns:Vec::new(),
            header:Vec::new(),
            title: "table",
        }
    }

    pub fn form_header(h:Vec<&'a str>) -> TableRender<'a> {
        TableRender {
            state:TableState::default(),
            columns:Vec::new(),
            header:h,
            title: "table",
        }
    }

    pub fn form_items(items:Vec<Vec<&'a str>>) -> TableRender<'a> {
        TableRender {
            state:TableState::default(),
            columns: items,
            header:Vec::new(),
            title: "table",
        }
    }

    pub fn set_header(mut self,h:Vec<&'a str>) -> Self {
       self.header = h;
        return self
    }

    pub fn set_state(mut self,state:TableState) -> Self{
        self.state = state;
        return  self
    }

    pub  fn set_title(mut self,title :&'a str) -> Self {
        self.title = title;
        return self
    }

    pub fn set_items(mut self,items:Vec<Vec<&'a str>>) -> Self {
        self.columns = items;
        return self
    }

    pub fn add_item(mut self,it:Vec<&'a str>) -> Self {
        self.columns.push(it);
        return self
    }

    pub fn add_header(mut self,k:&'a str) -> Self {
        self.header.push(k);
        return self
    }

    pub fn get_items(self) -> Vec<Vec<&'a str>> {
        return self.columns
    }

    pub fn get_headers(self) -> Vec<&'a str> {
        return self.header
    }


    pub fn get_title(self) ->&'a str {
        return self.title
    }

    pub fn get_state(mut self) ->TableState {
        return self.state.clone()
    }

    pub fn render<B: Backend>(mut self, terminal: &mut Terminal<B>) {
        terminal.draw(|f| self.draw(f)).unwrap();
    }

    fn draw<B: Backend>(self,f:&mut Frame<B>) {
        let rects =  new_default_layout(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = self.header
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));

        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = self.columns.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(*c));
            Row::new(cells).height(height as u16).bottom_margin(1)
        });

        let table_widget = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(self.title))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);
        f.render_stateful_widget(table_widget, rects[0], &mut self.get_state());
    }

}

#[allow(unused)]
pub fn show_table(table : HashMap<&str,Vec<&str>>) {
    let stdout = io::stdout();
    let mut headers  = Vec::new();
    let mut columns = Vec::new();
    table.iter().for_each(|(i, x)| {
        headers.push(i.clone());
        columns.push(x);
    });
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut render = TableRender::new();
    render = render.set_header(headers);
    for column in columns {
       render = render.add_item(column.to_vec());
    }
    render.render(&mut terminal)
}

fn new_default_layout(r:Rect) ->Vec<Rect> {
    Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(r)
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::output::output::show_table;
    #[test]
    fn test_show_table() {
        let  mut app = HashMap::new();
        app.insert("c1",Vec::from(vec!["1","2","3"]));
        app.insert("c2",Vec::from(vec!["10","20","60"]));
        app.insert("c3",Vec::from(vec!["10","20","60"]));
        app.insert("c4",Vec::from(vec!["10","20","60"]));
        app.insert("c5",Vec::from(vec!["10","20","60"]));
        show_table(app);
    }
}