- 360줄 참


```c
/* Write formatted output to STREAM.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern int fprintf (FILE *__restrict __stream,
		    const char *__restrict __format, ...) __nonnull ((1));
/* Write formatted output to stdout.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern int printf (const char *__restrict __format, ...);
/* Write formatted output to S.  */
extern int sprintf (char *__restrict __s,
		    const char *__restrict __format, ...) __THROWNL;

/* Write formatted output to S from argument list ARG.


/* STRAME에 형식화된 출력을 작성합니다.

   이 함수는 가능한 취소 지점이므로 그렇지 않습니다
   __THROW로 표시됨. */
외부 int fprintf (FILE *__restrict __stream,
		    const char *___restrict __format, ...) __nonnull (((1));
/* 형식화된 출력을 stdout에 씁니다.

   이 함수는 가능한 취소 지점이므로 그렇지 않습니다
   __THROW로 표시됨. */
외부 int printf (const char *___restrict __format, ...);
/* 형식화된 출력을 S. */에 씁니다
외부 int sprintf (char *___restrict __s,
		    const char *__restrict __format, ...) __THROWNL;

/* 인수 목록 ARG에서 형식화된 출력을 S에 씁니다.
```

