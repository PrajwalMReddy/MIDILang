#ifndef MIDILANG_LOOP_H
#define MIDILANG_LOOP_H

#include "../Expression.h"
#include "../other/Statement.h"


namespace MIDILang {
    class Loop: public Action {
        public:
            Loop(Expression* iterations, Statement* statements);
            std::any accept(ActVisitor& visitor);

            Expression* getIterations();
            Statement* getStatements();

        private:
            Expression* iterations;
            Statement* statements;
    };
}


#endif
