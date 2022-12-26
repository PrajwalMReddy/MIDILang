#ifndef MIDILANG_PARSER_H
#define MIDILANG_PARSER_H

#include <vector>

#include "../lang/Token.h"
#include "../lang/ErrorHandler.h"
#include "../ast/Action.h"
#include "../ast/Declaration.h"
#include "../ast/Expression.h"
#include "../ast/Other.h"
#include "../ast/action/Assignment.h"
#include "../ast/action/Loop.h"
#include "../ast/action/Note.h"
#include "../ast/action/Play.h"
#include "../ast/declaration/Tune.h"
#include "../ast/declaration/VariableDecl.h"
#include "../ast/expression/Binary.h"
#include "../ast/expression/Literal.h"
#include "../ast/expression/VariableExpr.h"
#include "../ast/other/Program.h"
#include "../ast/other/Statement.h"


namespace MIDILang {
    class Parser {
        public:
            Parser(std::vector<Token> tokens, ErrorHandler* errors);
            Program* parse();

            // AST Node Functions

            Program* programNode();
            Statement* statementNode(TokenType until);

            Tune* tuneNode();
            VariableDecl* variableDeclarationNode();
            Loop* loopNode();
            Play* playNode();
            Note* noteNode();
            Assignment* assignmentNode();
            Expression* expressionNode();

        private:
            std::vector<Token> tokens;
            Program* program;
            int current;
            ErrorHandler* errors;

            // Helper Functions

            Token peek();
            bool check(TokenType ttype);

            Token advance();
            bool matchAdvance(TokenType ttype);
            void newError(std::string message, int line);
    };
}


#endif
