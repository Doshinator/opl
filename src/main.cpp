#include <print>
#include "expression.h"

int main() {
    std::println("---- Start -----");
    
    Expression* expr =
        new Add(
            new Num(2),
            new Multiply(
                new Num(3),
                new Num(4)
            )
        );

    std::println("Result = {}", expr->eval());
    delete expr;

    std::println("---- End -----");
    return 0;
}