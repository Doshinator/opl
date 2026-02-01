#include "s_expression.h"

SSymbol::SSymbol(const std::string& name) {
    this->name = name;
}

SNumber::SNumber(int val) {
    this->val = val;
}