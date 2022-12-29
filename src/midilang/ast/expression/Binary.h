#ifndef MIDILANG_BINARY_H
#define MIDILANG_BINARY_H

#include <string>

#include "Literal.h"


namespace MIDILang {
    class Binary: public Expression {
        public:
            Binary(Expression* lvalue, std::string opType, Expression* rvalue);
            std::any accept(ExprVisitor& visitor);

            Expression* getLValue();
            std::string getOpType();
            Expression* getRValue();

        private:
            Expression* lvalue;
            std::string opType;
            Expression* rvalue;
    };
}


#endif
