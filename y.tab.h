/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2021 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

#ifndef YY_YY_Y_TAB_H_INCLUDED
# define YY_YY_Y_TAB_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int yydebug;
#endif

/* Token kinds.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    YYEMPTY = -2,
    YYEOF = 0,                     /* "end of file"  */
    YYerror = 256,                 /* error  */
    YYUNDEF = 257,                 /* "invalid token"  */
    ERROR = 258,                   /* ERROR  */
    ENDMARKER = 259,               /* ENDMARKER  */
    DURATION = 260,                /* DURATION  */
    NUMBER = 261,                  /* NUMBER  */
    FIRSTWEEK = 262,               /* FIRSTWEEK  */
    SECONDWEEK = 263,              /* SECONDWEEK  */
    THIRDWEEK = 264,               /* THIRDWEEK  */
    FOURTHWEEK = 265,              /* FOURTHWEEK  */
    FIFTHWEEK = 266,               /* FIFTHWEEK  */
    LASTWEEK = 267,                /* LASTWEEK  */
    SECONDLAST = 268,              /* SECONDLAST  */
    THIRDLAST = 269,               /* THIRDLAST  */
    FOURTHLAST = 270,              /* FOURTHLAST  */
    FIFTHLAST = 271,               /* FIFTHLAST  */
    MINUTECOMMAND = 272,           /* MINUTECOMMAND  */
    DAILYCOMMAND = 273,            /* DAILYCOMMAND  */
    WEEKLYCOMMAND = 274,           /* WEEKLYCOMMAND  */
    MONTHPOSCOMMAND = 275,         /* MONTHPOSCOMMAND  */
    MONTHDAYCOMMAND = 276,         /* MONTHDAYCOMMAND  */
    YEARDAYCOMMAND = 277,          /* YEARDAYCOMMAND  */
    YEARMONTHCOMMAND = 278,        /* YEARMONTHCOMMAND  */
    LASTDAY = 279,                 /* LASTDAY  */
    SUNDAY = 280,                  /* SUNDAY  */
    MONDAY = 281,                  /* MONDAY  */
    TUESDAY = 282,                 /* TUESDAY  */
    WEDNESDAY = 283,               /* WEDNESDAY  */
    THURSDAY = 284,                /* THURSDAY  */
    FRIDAY = 285,                  /* FRIDAY  */
    SATURDAY = 286,                /* SATURDAY  */
    DATE = 287                     /* DATE  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif
/* Token kinds.  */
#define YYEMPTY -2
#define YYEOF 0
#define YYerror 256
#define YYUNDEF 257
#define ERROR 258
#define ENDMARKER 259
#define DURATION 260
#define NUMBER 261
#define FIRSTWEEK 262
#define SECONDWEEK 263
#define THIRDWEEK 264
#define FOURTHWEEK 265
#define FIFTHWEEK 266
#define LASTWEEK 267
#define SECONDLAST 268
#define THIRDLAST 269
#define FOURTHLAST 270
#define FIFTHLAST 271
#define MINUTECOMMAND 272
#define DAILYCOMMAND 273
#define WEEKLYCOMMAND 274
#define MONTHPOSCOMMAND 275
#define MONTHDAYCOMMAND 276
#define YEARDAYCOMMAND 277
#define YEARMONTHCOMMAND 278
#define LASTDAY 279
#define SUNDAY 280
#define MONDAY 281
#define TUESDAY 282
#define WEDNESDAY 283
#define THURSDAY 284
#define FRIDAY 285
#define SATURDAY 286
#define DATE 287

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 52 "cde/lib/csa/reparser.y"

		int		 number;
		RepeatEvent	*re;
		NumberList	*nl;
		WeekDay		 weekday;
		WeekNumber	 weeknum;
		DayTime		*dt;
		DayTimeList	*dtl;
		WeekDayTimeList	*wdtl;
		time_t		 enddate;
		char		 date[64];
	

#line 145 "y.tab.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE yylval;


int yyparse (void);


#endif /* !YY_YY_Y_TAB_H_INCLUDED  */
