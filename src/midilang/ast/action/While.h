#ifndef MIDILANG_WHILE_H
#define MIDILANG_WHILE_H

#include "../Expression.h"
#include "../other/Statement.h"


namespace MIDILang {
    class While: public Action {
        public:
            While(Expression* condition, Statement* statements);
            std::any accept(ActVisitor& visitor);

            Expression* getCondition();
            Statement* getStatements();

        private:
            Expression* condition;
            Statement* statements;
    };
}


#endif
