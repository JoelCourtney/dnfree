grammar quantities;

@parser::header {

}

@lexer::header {

}

@parser::members {

}

@lexer::members {

}

time_quantity returns [Quantity<Time> result]
    : time_component                        { $result = $time_component.result; }
    | a=time_component '+' b=time_component { $result = ($a.result).add($b.result); }
    | time_component '-' time_component     { $result = $a.result.subtract($b.result); };

time_component returns [Quantity<Time> result]
    : e=expression u=time_unit;

time_unit returns [Time result]
    : ACTION        { $result = Time.ACTION; }
    | BONUS_ACTION  { $result = Time.BONUS_ACTION; }
    | REACTION      { $result = Time.REACTION; }
    | SECOND        { $result = Time.SECOND; }
    | MINUTE        { $result = Time.MINUTE; }
    | HOUR          { $result = Time.HOUR; }
    | DAY           { $result = Time.DAY; }
    | SHORT_REST    { $result = Time.SHORT_REST; }
    | LONG_REST     { $result = Time.LONG_REST; };

distance_quantity returns [Quantity<Distance> result]
    : distance_component                        { $result = $distance_component.result; }
    | a=distance_component '+' b=distance_component { $result = ($a.result).add($b.result); }
    | distance_component '-' distance_component     { $result = $a.result.subtract($b.result); };

distance_component returns [Quantity<Distance> result]
    : e=expression u=distance_unit;

distance_unit returns [Distance result]
    : FOOT      { $result = Distance.FOOT; }
    | MILE      { $result = Distance.MILE; }
    | SPACE     { $result = Distance.SPACE; };

damage_quantity returns [Quantity<Damage> result]
    : damage_component                        { $result = $damage_component.result; }
    | a=damage_component '+' b=damage_component { $result = ($a.result).add($b.result); }
    | damage_component '-' damage_component     { $result = $a.result.subtract($b.result); };

damage_component returns [Quantity<Damage> result]
    : e=expression u=damage_unit;

damage_unit returns [Damage result]
    : ACID          { $result = Damage.ACID; }
    | BLUDGEONING   { $result = Damage.BLUDGEONING; }
    | COLD          { $result = Damage.COLD; }
    | FIRE          { $result = Damage.FIRE; }
    | FORCE         { $result = Damage.FORCE; }
    | LIGHTNING     { $result = Damage.LIGHTNING; }
    | NECROTIC      { $result = Damage.NECROTIC; }
    | PIERCING      { $result = Damage.PIERCING; }
    | POISON        { $result = Damage.POISON; }
    | PSYCHIC       { $result = Damage.PSYCHIC; }
    | RADIANT       { $result = Damage.RADIANT; }
    | SLASHING      { $result = Damage.SLASHING; }
    | THUNDER       { $result = Damage.THUNDER; };

expression returns [Expression result]
    : l=expression bop=( '*' | '/' | '/+' | '/-' ) r=expression
        { $result = BinaryOp(Binary.from_string($bop), $l, $r); }
    | l=expression bop=( '+' | '-' ) r=expression
        { $result = BinaryOp(Binary.from_string($bop), $l, $r); }
    | n=NUMBER { $result = Number($n); }
    | d=DICE { $result = Dice($d); }
    | i=IDENTIFIER { $result = Identifier($i); };

fragment DIGIT : [0-9];
fragment LETTER : [A-Z] | [a-z] | '_';

fragment A : 'a' | 'A';
fragment B : 'b' | 'B';
fragment C : 'c' | 'C';
fragment D : 'd' | 'D';
fragment E : 'e' | 'E';
fragment F : 'f' | 'F';
fragment G : 'g' | 'G';
fragment H : 'h' | 'H';
fragment I : 'i' | 'I';
fragment J : 'j' | 'J';
fragment K : 'k' | 'K';
fragment L : 'l' | 'L';
fragment M : 'm' | 'M';
fragment N : 'n' | 'N';
fragment O : 'o' | 'O';
fragment P : 'p' | 'P';
fragment Q : 'q' | 'Q';
fragment R : 'r' | 'R';
fragment S : 's' | 'S';
fragment T : 't' | 'T';
fragment U : 'u' | 'U';
fragment V : 'v' | 'V';
fragment W : 'w' | 'W';
fragment X : 'x' | 'X';
fragment Y : 'y' | 'Y';
fragment Z : 'z' | 'Z';

ACTION : A C T I O N S?;
BONUS_ACTION : B O N U S ' ' A C T I O N S?;
REACTION : R E A C T I O N S?;
SECOND : S E C (O N D)? S?;
MINUTE : M I N (U T E)? S?;
HOUR : H O U R S? | H R S;
DAY : D A Y S?;
LONG_REST : L O N G ' ' R E S T S?;
SHORT_REST : S H O R T ' ' R E S T S?;
ACID : A C I D;
BLUDGEONING : B L U D G E O N I N G;
COLD : C O L D;
FIRE : F I R E;
FORCE : F O R C E;
LIGHTNING : L I G H T N I N G;
NECROTIC : N E C R O T I C;
PIERCING : P I E R C I N G;
POISON : P O I S O N;
PSYCHIC : P S Y C H I C;
RADIANT : R A D I A N T;
SLASHING : S L A S H I N G;
THUNDER : T H U N D E R;
FOOT : F (E E | O O) T;
MILE : M I L E S?;
SPACE : S P A C E S?;


NUMBER : DIGIT+;
DICE : DIGIT+ D DIGIT+;
IDENTIFIER : LETTER (LETTER | DIGIT)*;

WS : [ \t\u000c]+ -> skip;
