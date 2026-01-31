#include "test_infra.h"
#include "../src/expression.h"

class Test {
    public:
        void expression_num_evaluates_to_expected_int() {
            // Arrange
            int expected = 5;
            Expression *expr = new Num(5);

            // Act
            int actual = expr->eval();
            
            // Assert
            TEST_EXPRESSION(
                "expression_num_evaluates_to_expected_int",
                actual, 
                expected
            );

            delete expr;
        }

        void expression_add_evaluates_to_num() {
            // Arrange
            int expected = 12;
            Expression *expr = new Add(new Num(6), new Num(6));

            // Act
            int actual = expr->eval();

            // Assert
            TEST_EXPRESSION(
                "expression_add_evaluates_to_num", 
                actual,
                expected
            );

            delete expr;
        }

        void expression_add_evaluates_negative_num_success() {
            // Arrange
            int expected = -10;
            Expression *expr = new Add(new Num(-5), new Num(-5));

            // Act
            int actual = expr->eval();

            // Assert
            TEST_EXPRESSION(
                "expression_add_evaluates_negative_num_success", 
                actual,
                expected
            );

            delete expr;
        }

        void expression_multiply_evaluates_to_num() {
            // Arrange
            int expected = 12;
            Expression *expr = new Multiply(new Num(3), new Num(4));

            // Act
            int actual = expr->eval();

            // Assert
            TEST_EXPRESSION(
                "expression_multiply_evaluates_to_num", 
                actual,
                expected
            );

            delete expr;
        }

        void expression_multiply_evaluates_to_negative_nums() {
            // Arrange
            int expected = -72;
            Expression *expr = new Multiply(new Num(-6), new Num(12));

            // Act
            int actual = expr->eval();

            // Assert
            TEST_EXPRESSION(
                "expression_multiply_evaluates_to_negative_nums", 
                actual,
                expected
            );

            delete expr;
        }
};

int main() {
    Test t;
    t.expression_num_evaluates_to_expected_int();
    t.expression_add_evaluates_negative_num_success();
    t.expression_add_evaluates_to_num();
    t.expression_multiply_evaluates_to_num();
    t.expression_multiply_evaluates_to_negative_nums();

    TEST_SUMMARY();
    return 0;
}