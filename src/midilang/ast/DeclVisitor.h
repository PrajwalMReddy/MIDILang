#ifndef MIDILANG_DECLVISITOR_H
#define MIDILANG_DECLVISITOR_H

#include <any>


namespace MIDILang {
    class DeclVisitor {
        public:
            virtual std::any visitTuneDeclaration(class Tune* declaration) = 0;
            virtual std::any visitVariableDeclaration(class VariableDecl* declaration) = 0;
    };
}


#endif
