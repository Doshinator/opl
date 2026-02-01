#include <memory>
#include <print>
#include "expression.h"
#include "s_expression.h"

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

    auto inner = std::make_unique<SList>();
    inner->elements.push_back(std::make_unique<SSymbol>("*"));
    inner->elements.push_back(std::make_unique<SNumber>(5));
    inner->elements.push_back(std::make_unique<SNumber>(2));

    auto outer = std::make_unique<SList>();
    outer->elements.push_back(std::make_unique<SSymbol>("+"));
    outer->elements.push_back(std::make_unique<SNumber>(2));
    outer->elements.push_back(std::move(inner));
    

    std::println("Result = {}", expr->eval());
    delete expr;

    std::println("---- End -----");
    return 0;
}