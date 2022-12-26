#ifndef MIDILANG_VARIABLEEXPR_H
#define MIDILANG_VARIABLEEXPR_H

#include "../Expression.h"
#include "../../lang/Token.h"


namespace MIDILang {
    class VariableExpr: public Expression {
        public:
            VariableExpr(Token name);
            std::any accept(ExprVisitor& visitor);

            Token getName();

        private:
            Token name;
    };
}


#endif
