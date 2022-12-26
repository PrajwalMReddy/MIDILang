#ifndef MIDILANG_TUNE_H
#define MIDILANG_TUNE_H

#include <vector>

#include "../../lang/Token.h"
#include "../other/Statement.h"


namespace MIDILang {
    class Tune: public Declaration {
        public:
            Tune(Token name, std::vector<Token> parameters, Statement* statement);
            std::any accept(DeclVisitor& visitor);

            Token getName();
            std::vector<Token> getParameters();
            Statement* getStatement();

        private:
            Token name;
            std::vector<Token> parameters;
            Statement* statement;
    };
}


#endif
