#include "Statement.h"


MIDILang::Statement::Statement(std::vector<Declaration*>* declarationStatements, std::vector<Action*>* actionStatements) {
    this->declarationStatements = declarationStatements;
    this->actionStatements = actionStatements;
}

std::any MIDILang::Statement::accept(MIDILang::OtherVisitor& visitor) {
    return visitor.visitStatementNode(this);
}

std::vector<MIDILang::Declaration*>* MIDILang::Statement::getDeclarationStatements() {
    return this->declarationStatements;
}

std::vector<MIDILang::Action*>* MIDILang::Statement::getActionStatements() {
    return this->actionStatements;
}
