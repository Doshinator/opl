#include "s_expression.h"

SExpression::SExpression() {}

SSymbol::SSymbol(std::string name) {
    this->name = name;
}

SNumber::SNumber(int val) {
    this->val = val;
}

SList::SList(std::vector<std::unique_ptr<SExpression>> *expr) {
    this->s_list = expr;
};