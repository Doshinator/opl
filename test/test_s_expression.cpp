#include "../src/s_expression.h"
#include <memory>

class Test {
    public:
        void test_sexpr_num() {
            // Arrange
            auto sexpr = sexpr_num(5);

            // Act

            // Assert
        }

        void test_sexpr_add() {
            // Arrange
            auto sexpr = sexpr_add(
                sexpr_num(3), 
                sexpr_num(5));

            // Act

            // Assert

        }

        void test_sexpr_multi() {
            // Arrange
            auto sexpr = sexpr_multiply(
                sexpr_num(6), 
                sexpr_num(4));

            // Act

            // Assert
        }

        // -------
        // helpers
        // -------
        std::unique_ptr<SList> sexpr_num(int n) {
            auto list = std::make_unique<SList>();
            list->elements.push_back(std::make_unique<SNumber>(n));
            return list;
        }

        std::unique_ptr<SExpression> sexpr_add(
            std::unique_ptr<SExpression> a,
            std::unique_ptr<SExpression> b
        ) {
            auto list = std::make_unique<SList>();
            list->elements.push_back(std::make_unique<SSymbol>("+"));
            list->elements.push_back(a);
            list->elements.push_back(b);
            return list;
        }

        std::unique_ptr<SExpression> sexpr_multiply(
            std::unique_ptr<SExpression> a,
            std::unique_ptr<SExpression> b
        ) {
            auto list = std::make_unique<SList>();
            list->elements.push_back(std::make_unique<SSymbol>("*"));
            list->elements.push_back(a);
            list->elements.push_back(b);
            return list;
        }
};