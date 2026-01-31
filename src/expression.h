#pragma once

class Expression {
    public:
        virtual ~Expression() = default;
        virtual int eval() = 0;
};

class Num : public Expression {
    private:
        int n;
    public:
        Num(int n);
        int eval() override;
};

class Add : public Expression {
    private:
        Expression *lhs;
        Expression *rhs;
    public:
        Add(Expression *lhs, Expression *rhs);
        ~Add();
        int eval() override;
        
};

class Multiply : public Expression {
    private:
        Expression *lhs;
        Expression *rhs;
    public:
        Multiply(Expression *lhs, Expression *rhs);
        ~Multiply();
        int eval() override;
};