#ifndef MIDILANG_SYMBOLTABLE_H
#define MIDILANG_SYMBOLTABLE_H

#include <string>
#include <unordered_map>

#include "../ast/declaration/Tune.h"
#include "TuneVar.h"
#include "VariableVar.h"


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

            // Other Functions
            void dropAndDecrement();
            void incrementScope();
            void decrementScope();

        private:
            int scope;
            std::unordered_map<std::string, TuneVar> tunes;
            std::unordered_map<std::string, VariableVar> variables;
    };
}


#endif
