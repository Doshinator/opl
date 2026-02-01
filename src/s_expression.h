#include <vector>
#include <memory>

class SExpression {
    public:
        SExpression();
};

class SSymbol : public SExpression {
    private:
        std::string name;
    public:
        SSymbol(std::string name);
};

class SNumber : public SExpression {
    private:
        int val;
    public:
        SNumber(int val);
};

class SList : public SExpression {
    private:
        std::vector<std::unique_ptr<SExpression>> *s_list;
    public:
        SList(std::vector<std::unique_ptr<SExpression>> *expr);
};