#ifndef MIDILANG_VARIABLEDECL_H
#define MIDILANG_VARIABLEDECL_H

#include "../Expression.h"
#include "../../lang/Token.h"
#include "../Declaration.h"


namespace MIDILang {
    class VariableDecl: public Declaration {
        public:
            VariableDecl(Token name, Expression* value);
            std::any accept(DeclVisitor& visitor);

            Token getName();
            Expression* getValue();

        private:
            Token name;
            Expression* value;
    };
}


#endif
