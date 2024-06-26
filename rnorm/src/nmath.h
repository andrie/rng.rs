#define ISNAN(x) (isnan(x)!=0)

#define give_log log_p
# define R_FINITE(x)    isfinite(x)
#define ML_WARN_return_NAN { ML_WARNING(ME_DOMAIN, ""); return ML_NAN; }
#define ML_POSINF	INFINITY
#define ML_NEGINF	-INFINITY
#define ML_NAN		R_NaN
#define R_DT_qIv(p)	(log_p ? (lower_tail ? exp(p) : - expm1(p)) \
			       : R_D_Lval(p))
#define M_2PI		6.283185307179586476925286766559	/* 2*pi */

#define ML_WARNING(x, s) { \
   if(x > ME_DOMAIN) { \
       char *msg = ""; \
       switch(x) { \
       case ME_DOMAIN: \
	   msg = _("argument out of domain in '%s'\n");	\
	   break; \
       case ME_RANGE: \
	   msg = _("value out of range in '%s'\n");	\
	   break; \
       case ME_NOCONV: \
	   msg = _("convergence failed in '%s'\n");	\
	   break; \
       case ME_PRECISION: \
	   msg = _("full precision may not have been achieved in '%s'\n"); \
	   break; \
       case ME_UNDERFLOW: \
	   msg = _("underflow occurred in '%s'\n");	\
	   break; \
       } \
       MATHLIB_WARNING(msg, s); \
   } \
}

#define ME_NONE		0
/*	no error */
#define ME_DOMAIN	1
/*	argument out of domain */
#define ME_RANGE	2
/*	value out of range */
#define ME_NOCONV	4
/*	process did not converge */
#define ME_PRECISION	8
/*	does not have "full" precision */
#define ME_UNDERFLOW	16
/*	and underflow occurred (important for IEEE)*/

#define M_SQRT_2PI	2.50662827463100050241576528481104525301  /* sqrt(2*pi) */

#define R_DT_CIv(p)	(log_p ? (lower_tail ? -expm1(p) : exp(p)) \
			       : R_D_Cval(p))



/* Do the boundaries exactly for q*() functions :
 * Often  _LEFT_ = ML_NEGINF , and very often _RIGHT_ = ML_POSINF;
 *
 * R_Q_P01_boundaries(p, _LEFT_, _RIGHT_)  :<==>
 *
 *     R_Q_P01_check(p);
 *     if (p == R_DT_0) return _LEFT_ ;
 *     if (p == R_DT_1) return _RIGHT_;
 *
 * the following implementation should be more efficient (less tests):
 */
#define R_Q_P01_boundaries(p, _LEFT_, _RIGHT_)		\
    if (log_p) {					\
		if(p > 0)					\
	    	ML_WARN_return_NAN;				\
		if(p == 0) /* upper bound*/			\
			return lower_tail ? _RIGHT_ : _LEFT_;	\
		if(p == -INFINITY)				\
			return lower_tail ? _LEFT_ : _RIGHT_;	\
    }							\
    else { /* !log_p */					\
		if(p < 0 || p > 1)				\
			ML_WARN_return_NAN;				\
		if(p == 0)					\
			return lower_tail ? _LEFT_ : _RIGHT_;	\
		if(p == 1)					\
			return lower_tail ? _RIGHT_ : _LEFT_;	\
    }

#define warning(...) // Do not use warning in this file
#define MATHLIB_WARNING(fmt,x)		warning(fmt,x)

/* Use 0.5 - p + 0.5 to perhaps gain 1 bit of accuracy */
#define R_D_Lval(p)	(lower_tail ? (p) : (0.5 - (p) + 0.5))	/*  p  */
#define R_D_Cval(p)	(lower_tail ? (0.5 - (p) + 0.5) : (p))	/*  1 - p */
#define _(String) (String)