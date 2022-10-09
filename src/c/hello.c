//
// Created by 何为 on 2022/10/9.
//

#include "hello.h"
#include <stdio.h>

/// 获取c模块名
const char* get_module_name() {
    return MODULE_NAME;
}

int max(int num1, int num2)  {
    return num1 > num2 ? num1 : num2;
}

void display_name() {
    printf("hello clang for rust %s", get_module_name());
}



