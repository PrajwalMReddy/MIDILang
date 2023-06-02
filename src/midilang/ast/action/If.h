#ifndef MIDILANG_IF_H
#define MIDILANG_IF_H

#include "../Expression.h"
#include "../other/Statement.h"


namespace MIDILang {
    class If: public Action {
        public:
            If(Expression* condition, Statement* ifStatements, Statement* elseStatements);
            std::any accept(ActVisitor& visitor);

            Expression* getCondition();
            Statement* getIfStatements();
            Statement* getElseStatements();

            bool hasElseCondition();

        private:
            Expression* condition;
            Statement* ifStatements;
            Statement* elseStatements;
    };
}


#endif
