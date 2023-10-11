import{r as Z,s as O,n as M,i as ee,c as te,u as ne,g as se,d as le,e as ie}from"../chunks/scheduler.e8b33e29.js";import{d as T,S as N,i as P,g as p,h as k,x as B,k as _,a as V,f as m,s as j,j as x,c as A,y as w,e as W,m as ae,n as oe,r as q,u as K,v as R,t as S,b as re,w as G,z as ce,A as de,p as fe}from"../chunks/index.a5583998.js";function J(i){return(i==null?void 0:i.length)!==void 0?i:Array.from(i)}function ue(i,e){i.d(1),e.delete(i.key)}function he(i,e,l,t,a,c,s,n,r,f,D,d){let o=i.length,u=c.length,y=o;const $={};for(;y--;)$[i[y].key]=y;const E=[],L=new Map,g=new Map,h=[];for(y=u;y--;){const v=d(a,c,y),C=l(v);let I=s.get(C);I?t&&h.push(()=>I.p(v,e)):(I=f(C,v),I.c()),L.set(C,E[y]=I),C in $&&g.set(C,Math.abs(y-$[C]))}const b=new Set,z=new Set;function H(v){T(v,1),v.m(n,D),s.set(v.key,v),D=v.first,u--}for(;o&&u;){const v=E[u-1],C=i[o-1],I=v.key,F=C.key;v===C?(D=v.first,o--,u--):L.has(F)?!s.has(I)||b.has(I)?H(v):z.has(F)?o--:g.get(I)>g.get(F)?(z.add(I),H(v)):(b.add(F),o--):(r(C,s),o--)}for(;o--;){const v=i[o];L.has(v.key)||r(v,s)}for(;u;)H(E[u-1]);return Z(h),E}function ve({url:i}){return{url:i.pathname}}const xe=Object.freeze(Object.defineProperty({__proto__:null,load:ve},Symbol.toStringTag,{value:"Module"}));function _e(i){let e,l='<div class="headerButton svelte-1ilknhg"><a href="/">Home</a></div> <h1 class="svelte-1ilknhg">Hello! Welcome to my Svelte project!</h1> <div class="headerButton svelte-1ilknhg"><a href="/login">Login</a></div>';return{c(){e=p("header"),e.innerHTML=l,this.h()},l(t){e=k(t,"HEADER",{class:!0,"data-svelte-h":!0}),B(e)!=="svelte-ww7wc3"&&(e.innerHTML=l),this.h()},h(){_(e,"class","svelte-1ilknhg")},m(t,a){V(t,e,a)},p:M,i:M,o:M,d(t){t&&m(e)}}}class me extends N{constructor(e){super(),P(this,e,null,_e,O,{})}}function Q(i,e,l){const t=i.slice();return t[1]=e[l],t}function ge(i){let e,l=" |";return{c(){e=p("span"),e.textContent=l},l(t){e=k(t,"SPAN",{"data-svelte-h":!0}),B(e)!=="svelte-z9nxu8"&&(e.textContent=l)},m(t,a){V(t,e,a)},d(t){t&&m(e)}}}function U(i,e){let l,t,a,c=e[1].name+"",s,n=e[1].id!==1&&ge();return{key:i,first:null,c(){l=W(),n&&n.c(),t=j(),a=p("a"),s=ae(c),this.h()},l(r){l=W(),n&&n.l(r),t=A(r),a=k(r,"A",{href:!0});var f=x(a);s=oe(f,c),f.forEach(m),this.h()},h(){_(a,"href",e[1].link),this.first=l},m(r,f){V(r,l,f),n&&n.m(r,f),V(r,t,f),V(r,a,f),w(a,s)},p(r,f){e=r},d(r){r&&(m(l),m(t),m(a)),n&&n.d(r)}}}function pe(i){let e,l,t,a="Keep in Touch",c,s,n=[],r=new Map,f=J(i[0]);const D=d=>d[1].id;for(let d=0;d<f.length;d+=1){let o=Q(i,f,d),u=D(o);r.set(u,n[d]=U(u,o))}return{c(){e=p("footer"),l=p("div"),t=p("h1"),t.textContent=a,c=j(),s=p("div");for(let d=0;d<n.length;d+=1)n[d].c();this.h()},l(d){e=k(d,"FOOTER",{class:!0});var o=x(e);l=k(o,"DIV",{id:!0});var u=x(l);t=k(u,"H1",{class:!0,"data-svelte-h":!0}),B(t)!=="svelte-gcqgyj"&&(t.textContent=a),c=A(u),s=k(u,"DIV",{id:!0,class:!0});var y=x(s);for(let $=0;$<n.length;$+=1)n[$].l(y);y.forEach(m),u.forEach(m),o.forEach(m),this.h()},h(){_(t,"class","svelte-1jdk9g7"),_(s,"id","ContactInfo"),_(s,"class","svelte-1jdk9g7"),_(l,"id","Links"),_(e,"class","svelte-1jdk9g7")},m(d,o){V(d,e,o),w(e,l),w(l,t),w(l,c),w(l,s);for(let u=0;u<n.length;u+=1)n[u]&&n[u].m(s,null)},p(d,[o]){o&1&&(f=J(d[0]),n=he(n,o,D,1,d,f,r,s,ue,U,null,Q))},i:M,o:M,d(d){d&&m(e);for(let o=0;o<n.length;o+=1)n[o].d()}}}function ke(i){return[[{id:1,name:"GitHub",link:"https://github.com/kelvin1095"},{id:2,name:"LinkedIn",link:"https://www.linkedin.com/in/kelvin-w-14679ab4"},{id:3,name:"Instagram",link:"https://www.instagram.com/negativeprogress"},{id:4,name:"Threads",link:"https://www.threads.net/@negativeprogress"}]]}class ye extends N{constructor(e){super(),P(this,e,ke,pe,O,{})}}function we(i){let e,l='<ul><li><a href="/">Home</a></li> <li><a href="/pokemon">Pokemon</a></li> <li><a href="/koi">Koi Fish</a></li> <li><a href="/about">About</a></li></ul>';return{c(){e=p("nav"),e.innerHTML=l,this.h()},l(t){e=k(t,"NAV",{id:!0,"data-svelte-h":!0}),B(e)!=="svelte-hnxz4o"&&(e.innerHTML=l),this.h()},h(){_(e,"id","navBox")},m(t,a){V(t,e,a)},p:M,i:M,o:M,d(t){t&&m(e)}}}class $e extends N{constructor(e){super(),P(this,e,null,we,O,{})}}function X(i,{delay:e=0,duration:l=400,easing:t=ee}={}){const a=+getComputedStyle(i).opacity;return{delay:e,duration:l,easing:t,css:c=>`opacity: ${c*a}`}}function Y(i){let e,l,t,a;const c=i[2].default,s=te(c,i,i[1],null);return{c(){e=p("div"),s&&s.c(),this.h()},l(n){e=k(n,"DIV",{id:!0});var r=x(e);s&&s.l(r),r.forEach(m),this.h()},h(){_(e,"id","transition")},m(n,r){V(n,e,r),s&&s.m(e,null),a=!0},p(n,r){s&&s.p&&(!a||r&2)&&ne(s,c,n,n[1],a?le(c,n[1],r,null):se(n[1]),null)},i(n){a||(T(s,n),n&&ie(()=>{a&&(t&&t.end(1),l=ce(e,X,{delay:300,duration:300}),l.start())}),a=!0)},o(n){S(s,n),l&&l.invalidate(),n&&(t=de(e,X,{delay:0,duration:300})),a=!1},d(n){n&&m(e),s&&s.d(n),n&&t&&t.end()}}}function be(i){let e,l,t,a,c,s,n,r,f,D=i[0].url,d,o,u="<p>something</p>",y,$,E,L;t=new me({}),n=new $e({});let g=Y(i);return E=new ye({}),{c(){e=p("div"),l=p("div"),q(t.$$.fragment),a=j(),c=p("div"),s=p("div"),q(n.$$.fragment),r=j(),f=p("div"),g.c(),d=j(),o=p("div"),o.innerHTML=u,y=j(),$=p("div"),q(E.$$.fragment),this.h()},l(h){e=k(h,"DIV",{id:!0,class:!0});var b=x(e);l=k(b,"DIV",{id:!0});var z=x(l);K(t.$$.fragment,z),z.forEach(m),a=A(b),c=k(b,"DIV",{id:!0,class:!0});var H=x(c);s=k(H,"DIV",{id:!0,class:!0});var v=x(s);K(n.$$.fragment,v),v.forEach(m),r=A(H),f=k(H,"DIV",{id:!0,class:!0});var C=x(f);g.l(C),C.forEach(m),d=A(H),o=k(H,"DIV",{id:!0,class:!0,"data-svelte-h":!0}),B(o)!=="svelte-1k0e8w9"&&(o.innerHTML=u),H.forEach(m),y=A(b),$=k(b,"DIV",{id:!0});var I=x($);K(E.$$.fragment,I),I.forEach(m),b.forEach(m),this.h()},h(){_(l,"id","header"),_(s,"id","leftColumn"),_(s,"class","svelte-catfga"),_(f,"id","mainColumn"),_(f,"class","svelte-catfga"),_(o,"id","rightColumn"),_(o,"class","svelte-catfga"),_(c,"id","mainContent"),_(c,"class","svelte-catfga"),_($,"id","footer"),_(e,"id","root"),_(e,"class","svelte-catfga")},m(h,b){V(h,e,b),w(e,l),R(t,l,null),w(e,a),w(e,c),w(c,s),R(n,s,null),w(c,r),w(c,f),g.m(f,null),w(c,d),w(c,o),w(e,y),w(e,$),R(E,$,null),L=!0},p(h,[b]){b&1&&O(D,D=h[0].url)?(fe(),S(g,1,1,M),re(),g=Y(h),g.c(),T(g,1),g.m(f,null)):g.p(h,b)},i(h){L||(T(t.$$.fragment,h),T(n.$$.fragment,h),T(g),T(E.$$.fragment,h),L=!0)},o(h){S(t.$$.fragment,h),S(n.$$.fragment,h),S(g),S(E.$$.fragment,h),L=!1},d(h){h&&m(e),G(t),G(n),g.d(h),G(E)}}}function Ce(i,e,l){let{$$slots:t={},$$scope:a}=e,{data:c}=e;return i.$$set=s=>{"data"in s&&l(0,c=s.data),"$$scope"in s&&l(1,a=s.$$scope)},[c,a,t]}class He extends N{constructor(e){super(),P(this,e,Ce,be,O,{data:0})}}export{He as component,xe as universal};
