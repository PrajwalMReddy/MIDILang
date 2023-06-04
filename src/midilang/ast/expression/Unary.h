#ifndef MIDILANG_UNARY_H
#define MIDILANG_UNARY_H

#include <string>

#include "Literal.h"


namespace MIDILang {
    class Unary: public Expression {
        public:
            Unary(std::string opType, Expression* value);
            std::any accept(ExprVisitor& visitor);

            std::string getOpType();
            Expression* getValue();

        private:
            std::string opType;
            Expression* value;
    };
}


#endif
