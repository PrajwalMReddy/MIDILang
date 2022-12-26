#ifndef MIDILANG_OTHERVISITOR_H
#define MIDILANG_OTHERVISITOR_H

#include <any>


namespace MIDILang {
    class OtherVisitor {
        public:
            virtual std::any visitProgramNode(class Program* node) = 0;
            virtual std::any visitStatementNode(class Statement* node) = 0;
    };
}


#endif
