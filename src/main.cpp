#include <print>

class Expression {
    public:
        virtual ~Expression() = default;
        virtual int eval() = 0;
};

class Num : public Expression {
    private:
        int n;
    public:
        Num(int n) { this->n = n; }
        int eval() { return n; }
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

        int eval() {
            return lhs->eval() +  rhs->eval();
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

        int eval() {
            return lhs->eval() * rhs->eval();
        }
};

int main() {
    std::println("---- Start -----");

    

    std::println("---- End -----");
    return 0;
}