#ifndef MIDILANG_EXPRVISITOR_H
#define MIDILANG_EXPRVISITOR_H

#include <any>


namespace MIDILang {
    class ExprVisitor {
        public:
            virtual std::any visitBinaryExpression(class Binary* expression) = 0;
            virtual std::any visitGroupingExpression(class Grouping* expression) = 0;
            virtual std::any visitLiteralExpression(class Literal* expression) = 0;
            virtual std::any visitUnaryExpression(class Unary* expression) = 0;
            virtual std::any visitVariableExpression(class VariableExpr* expression) = 0;
    };
}


#endif
