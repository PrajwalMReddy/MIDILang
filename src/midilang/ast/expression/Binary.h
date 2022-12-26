#ifndef MIDILANG_BINARY_H
#define MIDILANG_BINARY_H

#include <string>

#include "Literal.h"


namespace MIDILang {
    class Binary: public Expression {
        public:
            Binary(Literal lvalue, std::string opType, Literal rvalue);
            std::any accept(ExprVisitor& visitor);

            Literal getLValue();
            std::string getOpType();
            Literal getRValue();

        private:
            Literal lvalue;
            std::string opType;
            Literal rvalue;
    };
}


#endif
