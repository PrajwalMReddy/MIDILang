#ifndef MIDILANG_PROGRAM_H
#define MIDILANG_PROGRAM_H

#include "../Other.h"


namespace MIDILang {
    class Program: public Other {
        public:
            Program(Statement* statements);
            std::any accept(OtherVisitor& visitor);

            Statement* getStatements();

        private:
            Statement* statements;
    };
}


#endif
