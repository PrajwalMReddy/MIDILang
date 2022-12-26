#ifndef MIDILANG_STATEMENT_H
#define MIDILANG_STATEMENT_H

#include <vector>

#include "../Other.h"
#include "../Declaration.h"
#include "../Action.h"


namespace MIDILang {
    class Statement: public Other {
        public:
            Statement(std::vector<Declaration*>* declarationStatements, std::vector<Action*>* actionStatements);
            std::any accept(OtherVisitor& visitor);

            std::vector<Declaration*>* getDeclarationStatements();
            std::vector<Action*>* getActionStatements();

        private:
            std::vector<Declaration*>* declarationStatements;
            std::vector<Action*>* actionStatements;
    };
}


#endif
