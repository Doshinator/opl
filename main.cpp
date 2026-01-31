#include <print>

class Expression {};

class Num : public Expression {
    private:
        int n;
    public:
        Num(int n) { this->n = n; }
};

class Add : public Expression {
    private:
        Expression *lhs;
        Expression *rhs;
    public:
        Add(Expression *lhs, Expression *rhs) {
            this->lhs = lhs;
            this->rhs = rhs;
        }
        ~Add() {
            delete lhs;
            delete rhs;
        }
};

class Multiply : public Expression {
    private:
        Expression *lhs;
        Expression *rhs;
    public:
        Multiply(Expression *lhs, Expression *rhs) {
            this->lhs = lhs;
            this->rhs = rhs;
        }
        ~Multiply() {
            delete lhs;
            delete rhs;
        }
};


int main() {
    std::println("---- Start -----");
    
    std::println("---- End -----");
    return 0;
}