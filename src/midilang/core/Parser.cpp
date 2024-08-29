#include "Parser.h"


MIDILang::Parser::Parser(std::vector<Token> tokens, MIDILang::ErrorHandler* errors) {
    this->tokens = tokens;
    this->current = 0;
    this->errors = errors;
}

MIDILang::Program* MIDILang::Parser::parse() {
    this->program = programNode();
    return this->program;
}

MIDILang::Program* MIDILang::Parser::programNode() {
    return new Program(statementNode(TK_EOF));
}

MIDILang::Statement* MIDILang::Parser::statementNode(TokenType until) {
    auto* declarationStatements = new std::vector<Declaration*>();
    auto* actionStatements = new std::vector<Action*>();

    while (!check(until) && !check(TK_EOF)) {
        if (matchAdvance(TK_TUNE)) declarationStatements->push_back(tuneNode());
        else if (matchAdvance(TK_VAR)) declarationStatements->push_back(variableDeclarationNode());
        else if (matchAdvance(TK_LOOP)) actionStatements->push_back(loopNode());
        else if (matchAdvance(TK_WHILE)) actionStatements->push_back(whileNode());
        else if (matchAdvance(TK_IF)) actionStatements->push_back(ifNode());
        else if (matchAdvance(TK_PLAY)) actionStatements->push_back(playNode());
        else if (matchAdvance(TK_NOTE)) actionStatements->push_back(noteNode());
        else if (peek().ttype == TK_IDENTIFIER) actionStatements->push_back(assignmentNode());
        else newError("Unexpected Token: " + peek().literal, peek().line);
    }

    return new Statement(declarationStatements, actionStatements);
}

MIDILang::Tune* MIDILang::Parser::tuneNode() {
    Token identifier = advance();
    std::vector<Token> parameters = std::vector<Token>();
    Statement* statements;

    if (identifier.ttype != TK_IDENTIFIER) newError("Tune Names Must Be Identifiers", identifier.line);
    if (check(TK_COLON) && (peekN(1).ttype == TK_LEFT_BRACE)) newError("At Least One Parameter Expected After Colon", identifier.line);

    if (matchAdvance(TK_COLON)) while (!check(TK_LEFT_BRACE) && !check(TK_EOF)) {
        parameters.push_back(advance());
        if (parameters.back().ttype != TK_IDENTIFIER) newError("Tune Parameter Must Be An Identifier", identifier.line);
    }

    if (!matchAdvance(TK_LEFT_BRACE)) newError("An Opening Brace Was Expected Before The Tune Block", identifier.line);
    statements = statementNode(TK_RIGHT_BRACE);
    if (!matchAdvance(TK_RIGHT_BRACE)) newError("A Closing Brace Was Expected After The Tune Block", identifier.line);

    return new Tune(identifier, parameters, statements);
}

MIDILang::VariableDecl* MIDILang::Parser::variableDeclarationNode() {
    Token identifier = advance();
    Expression* value;

    if (identifier.ttype != TK_IDENTIFIER) newError("Tune Names Must Be Identifiers", identifier.line);
    if (!matchAdvance(TK_EQUAL)) newError("An Equals Sign Was Expected After The Variable Name", identifier.line);
    value = expressionNode();
    if (!matchAdvance(TK_SEMICOLON)) newError("A Semicolon Was Expected After The Variable Declaration", identifier.line);

    return new VariableDecl(identifier, value);
}

MIDILang::Loop* MIDILang::Parser::loopNode() {
    Expression* iterations;
    Statement* statements;

    if (!matchAdvance(TK_COLON)) newError("A Colon Was Expected After The Loop Keyword", peek().line);
    iterations = expressionNode();
    if (!matchAdvance(TK_LEFT_BRACE)) newError("An Opening Brace Was Expected Before The Iterations Block", peek().line);
    statements = statementNode(TK_RIGHT_BRACE);
    if (!matchAdvance(TK_RIGHT_BRACE)) newError("A Closing Brace Was Expected After The Loop Block", peek().line);

    return new Loop(iterations, statements);
}

MIDILang::While* MIDILang::Parser::whileNode() {
    Expression* condition;
    Statement* statements;

    if (!matchAdvance(TK_COLON)) newError("A Colon Was Expected After The While Keyword", peek().line);
    condition = expressionNode();
    if (!matchAdvance(TK_LEFT_BRACE)) newError("An Opening Brace Was Expected Before The Iterations Block", peek().line);
    statements = statementNode(TK_RIGHT_BRACE);
    if (!matchAdvance(TK_RIGHT_BRACE)) newError("A Closing Brace Was Expected After The While Block", peek().line);

    return new While(condition, statements);
}

MIDILang::If* MIDILang::Parser::ifNode() {
    Expression* condition;
    Statement* ifStatements;

    Statement* elseStatements = nullptr;
    If* nestedIfStatements = nullptr;

    if (!matchAdvance(TK_COLON)) newError("A Colon Was Expected After The If Keyword", peek().line);
    condition = expressionNode();
    if (!matchAdvance(TK_LEFT_BRACE)) newError("An Opening Brace Was Expected Before The If Block", peek().line);
    ifStatements = statementNode(TK_RIGHT_BRACE);
    if (!matchAdvance(TK_RIGHT_BRACE)) newError("A Closing Brace Was Expected After The If Block", peek().line);
    if (matchAdvance(TK_ELSE)) {
        if (matchAdvance(TK_IF)) {
            nestedIfStatements = ifNode();
        } else {
            if (!matchAdvance(TK_LEFT_BRACE)) newError("An Opening Brace Was Expected Before The Else Block", peek().line);
            elseStatements = statementNode(TK_RIGHT_BRACE);
            if (!matchAdvance(TK_RIGHT_BRACE)) newError("A Closing Brace Was Expected After The Else Block", peek().line);
        }
    }

    return new If(condition, ifStatements, elseStatements, nestedIfStatements);
}

MIDILang::Play* MIDILang::Parser::playNode() {
    Token identifier = advance();
    std::vector<Expression*> arguments = std::vector<Expression*>();

    if (matchAdvance(TK_COLON)) while (!check(TK_SEMICOLON) && !check(TK_EOF)) arguments.push_back(expressionNode());
    if (!matchAdvance(TK_SEMICOLON)) newError("A Semicolon Was Expected After The Tune Arguments", identifier.line);

    return new Play(identifier, arguments);
}

MIDILang::Note* MIDILang::Parser::noteNode() {
    int line = peek().line;

    Expression* note = expressionNode();
    Expression* duration = expressionNode();
    Expression* velocity = expressionNode();

    if (!matchAdvance(TK_SEMICOLON)) newError("A Semicolon Was Expected After The Note, Duration, And Velocity Expressions", peek().line);

    return new Note(note, duration, velocity, line);
}

MIDILang::Assignment* MIDILang::Parser::assignmentNode() {
    Token identifier = advance();
    Expression* value;

    if (!matchAdvance(TK_EQUAL)) newError("An Equals Sign Was Expected Before The Assignment Value", identifier.line);
    value = expressionNode();

    if (!matchAdvance(TK_SEMICOLON)) newError("A Semicolon Was Expected After Assignment Expression", peek().line);
    return new Assignment(identifier, value);
}

MIDILang::Expression* MIDILang::Parser::expressionNode() {
    return equalityNode();
}

MIDILang::Expression* MIDILang::Parser::equalityNode() {
    Expression* expression = comparisonNode();

    while (check(TK_COMPARE_EQUAL) || check(TK_COMPARE_NOT_EQUAL)) {
        Token op = advance();
        Expression* rvalue = comparisonNode();
        expression = new Binary(expression, op.literal, rvalue);
    }

    return expression;
}

MIDILang::Expression* MIDILang::Parser::comparisonNode() {
    Expression* expression = termNode();

    while (check(TK_GREATER) || check(TK_GREATER_EQUAL) || check(TK_LESSER) || check(TK_LESSER_EQUAL)) {
        Token op = advance();
        Expression* rvalue = termNode();
        expression = new Binary(expression, op.literal, rvalue);
    }

    return expression;
}

MIDILang::Expression* MIDILang::Parser::termNode() {
    Expression* expression = factorNode();

    while (check(TK_PLUS) || check(TK_MINUS)) {
        Token op = advance();
        Expression* rvalue = factorNode();
        expression = new Binary(expression, op.literal, rvalue);
    }

    return expression;
}

MIDILang::Expression* MIDILang::Parser::factorNode() {
    Expression* expression = unaryNode();

    while (check(TK_MULTIPLY) || check(TK_DIVIDE) || check(TK_MODULUS)) {
        Token op = advance();
        Expression* rvalue = unaryNode();
        expression = new Binary(expression, op.literal, rvalue);
    }

    return expression;
}

MIDILang::Expression* MIDILang::Parser::unaryNode() {
    if (check(TK_NOT) || check(TK_MINUS)) {
        Token op = advance();
        Expression* rvalue = unaryNode();
        return new Unary(op.literal, rvalue);
    } else if (matchAdvance(TK_PLUS)) {
        return unaryNode();
    } else {
        return primaryNode();
    }
}

MIDILang::Expression* MIDILang::Parser::primaryNode() {
    if (check(TK_IDENTIFIER)) {
        return new VariableExpr(advance());
    } else if (check(TK_NUMBER)) {
        return new Literal(advance());
    } else if (matchAdvance(TK_LEFT_PAR)) {
        Expression* expression = expressionNode();
        if (!matchAdvance(TK_RIGHT_PAR)) newError("A Closing Parenthesis Was Expected After The Parenthesized Expression", peek().line);
        return new Grouping(expression);
    }
}

MIDILang::Token MIDILang::Parser::peek() {
    return this->tokens[this->current];
}

MIDILang::Token MIDILang::Parser::peekN(int n) {
    return this->tokens[this->current + n];
}

bool MIDILang::Parser::check(TokenType ttype) {
    if (peek().ttype == ttype) return true;
    else return false;
}

MIDILang::Token MIDILang::Parser::advance() {
    this->current++;
    return this->tokens[this->current - 1];
}

bool MIDILang::Parser::matchAdvance(TokenType ttype) {
    if (peek().ttype == ttype) {
        advance();
        return true;
    }

    return false;
}

void MIDILang::Parser::newError(std::string message, int line) {
    this->errors->addError("Parser Error", message, line);

    // Prevents Ghost Errors And Parsing Breakdowns
    this->errors->displayIfHasErrors();
}
