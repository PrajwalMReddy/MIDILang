#ifndef MIDILANG_ASSIGNMENT_H
#define MIDILANG_ASSIGNMENT_H

#include "../Action.h"
#include "../Expression.h"
#include "../../lang/Token.h"


namespace MIDILang {
    class Assignment: public Action {
        public:
            Assignment(Token name, Expression* expr);
            std::any accept(ActVisitor& visitor);

            Token getName();
            Expression* getExpr();

        private:
            Token name;
            Expression* expr;
    };
}


#endif
