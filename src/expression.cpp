#include "expression.h"

// Num
Num::Num(int n) {
    this->n = n;
}

int Num::eval() {
    return this->n;
}

// Add
Add::Add(Expression *lhs, Expression *rhs) {
    this->lhs = lhs;
    this->rhs = rhs;
}

int Add::eval() {
    return lhs->eval() + rhs->eval();
}

Add::~Add() {
    delete lhs;
    delete rhs;
}

// Multiply

Multiply::Multiply(Expression *lhs, Expression *rhs) {
    this->lhs = lhs;
    this->rhs = rhs;
}

int Multiply::eval() {
    return lhs->eval() * rhs->eval();
}

Multiply::~Multiply() {
    delete lhs;
    delete rhs;
}