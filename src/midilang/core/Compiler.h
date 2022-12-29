#ifndef MIDILANG_COMPILER_H
#define MIDILANG_COMPILER_H

#include <fstream>

#include "../ast/other/Program.h"
#include "../lang/ErrorHandler.h"
#include "../variable//SymbolTable.h"
#include "../ast/ExprVisitor.h"
#include "../ast/expression/Literal.h"
#include "../ast/expression/VariableExpr.h"
#include "../ast/expression/Binary.h"
#include "../ast/declaration/VariableDecl.h"
#include "../ast/action/Assignment.h"
#include "../ast/action/Play.h"
#include "../ast/action/Note.h"
#include "../ast/action/Loop.h"


namespace MIDILang {
    class Compiler: public OtherVisitor, public DeclVisitor, public ActVisitor, public ExprVisitor {
        public:
            Compiler(Program* program, std::string path, ErrorHandler* errors);
            void compile();

        private:
            std::string path;
            std::vector<unsigned char> fileBytes;
            int fileLength;
            Program* program;
            SymbolTable* symbolTable;
            ErrorHandler* errors;

            // Core Functions
            void headerChunk();
            void trackChunk();
            void compileFile();
            void cleanUp();
            void writeFile();

            // Other Visitor Functions
            std::any visitProgramNode(Program* node) override;
            std::any visitStatementNode(Statement* node) override;

            // Declaration Visitor Functions
            std::any visitTuneDeclaration(Tune* declaration) override;
            std::any visitVariableDeclaration(VariableDecl* declaration) override;

            // Action Visitor Functions
            std::any visitAssignmentAction(Assignment* action) override;
            std::any visitLoopAction(Loop* action) override;
            std::any visitPlayAction(Play* action) override;
            std::any visitNoteAction(Note* action) override;

            // Expression Visitor Functions
            std::any visitBinaryExpression(Binary* expression) override;
            std::any visitLiteralExpression(Literal* expression) override;
            std::any visitVariableExpression(VariableExpr* expression) override;

            // Symbol Table API
            void addTune(Token identifier, Tune tuneStmt);
            Tune getTune(Token identifier);
            void addVariable(Token identifier, int value);
            int getVariable(Token identifier);
            void reassignVariable(Token identifier, int value);

            // Helper Functions
            std::vector<unsigned char> intToVLE(int duration, int line);
            void newError(std::string message, int line);
    };
}


#endif
