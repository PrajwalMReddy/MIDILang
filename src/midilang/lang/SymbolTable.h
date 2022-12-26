#ifndef MIDILANG_SYMBOLTABLE_H
#define MIDILANG_SYMBOLTABLE_H

#include <string>
#include <unordered_map>

#include "../ast/declaration/Tune.h"


namespace MIDILang {
    class SymbolTable {
        public:
            SymbolTable();

            // Tune Functions
            bool addTune(Token identifier, Tune tuneStmt);
            Tune getTune(Token identifier);
            void dropTune(Token identifier);
            bool hasTune(Token identifier);

            // Variable Functions
            bool addVariable(Token identifier, int value);
            int getVariable(Token identifier);
            bool reassignVariable(Token identifier, int value);
            void dropVariable(Token identifier);
            bool hasVariable(Token identifier);

        private:
            std::unordered_map<std::string, Tune> tunes;
            std::unordered_map<std::string, int> variables;
    };
}


#endif
