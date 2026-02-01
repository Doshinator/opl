#include <vector>
#include <memory>

class SExpression {
    public:
        virtual ~SExpression() = default;
        SExpression() = default;
};

class SSymbol : public SExpression {
    public:
        std::string name;
        SSymbol(const std::string& name);
};

class SNumber : public SExpression {
    public:
        int val;
        SNumber(int val);
};

class SList : public SExpression {
    public:
        std::vector<std::unique_ptr<SExpression>> elements;
        SList() = default;
};