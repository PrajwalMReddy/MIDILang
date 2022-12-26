#ifndef MIDILANG_LITERAL_H
#define MIDILANG_LITERAL_H

#include "../Expression.h"
#include "../../lang/Token.h"


namespace MIDILang {
    class Literal: public Expression {
        public:
            Literal(Token value);
            std::any accept(ExprVisitor& visitor);

            Token getValue();

        private:
            Token value;
    };
}


#endif
