import{H as Se}from"../chunks/control.f5b05b5f.js";import{s as Be,f as we,n as Le}from"../chunks/scheduler.59ff4fcc.js";import{S as Ie,i as Oe,s as o,g as e,m as v,B as je,f as l,c as n,h as a,j as s,n as c,x as dt,k as h,a as Me,y as t}from"../chunks/index.9fefb256.js";function Ce(r,_){return new Se(r,_)}new TextEncoder;const Pe=async({params:r})=>{let _=Number(r.id);const p=await fetch(`/api/pokemon/${_}`);if(!p.ok)throw Ce(p.status,"Not Found");return{pokemonInfo:p.json()}},We=Object.freeze(Object.defineProperty({__proto__:null,load:Pe},Symbol.toStringTag,{value:"Module"}));function Re(r){let _,p,d,u,L,J=r[0].pokedexnumber.toString().padStart(4,"0")+"",ot,nt,Jt=r[0].name+"",ht,vt,Kt=r[0].form?"("+r[0].form+")":"",ct,it,M,S,Qt,_t,b,B,Ut="<tr><th>Type</th></tr>",ut,K,Q,w,q,Xt=r[0].type1+"",ft,mt,Zt=(r[0].type2||"")+"",pt,Et,y,I,xt="<tr><th>Height</th> <th>Weight</th></tr>",Tt,U,z,H,te=r[0].height+"",bt,yt,Dt,k,ee=r[0].weight+"",gt,qt,zt,D,O,ae="<tr><th>Ability 1</th> <th>Ability 2</th> <th>Hidden Ability</th></tr>",Ht,X,E,j,le=r[0].ability1+"",kt,At,C,se=(r[0].ability2||"")+"",Lt,Mt,P,re=(r[0].hiddenability||"")+"",St,Bt,g,R,de="<tr><th>HP</th> <th>Attack</th> <th>Defense</th> <th>Sp. Atk</th> <th>Sp. Def</th> <th>Speed</th></tr>",wt,Z,i,Y,oe=r[0].hp+"",It,Ot,V,ne=r[0].att+"",jt,Ct,N,he=r[0].def+"",Pt,Rt,F,ve=r[0].spa+"",Yt,Vt,G,ce=r[0].spd+"",Nt,Ft,W,ie=r[0].spe+"",Gt;return document.title=_=r[0].name,{c(){p=o(),d=e("div"),u=e("h1"),L=v("#"),ot=v(J),nt=o(),ht=v(Jt),vt=o(),ct=v(Kt),it=o(),M=e("div"),S=e("img"),_t=o(),b=e("table"),B=e("thead"),B.innerHTML=Ut,ut=o(),K=e("tbody"),Q=e("tr"),w=e("td"),q=e("div"),ft=v(Xt),mt=o(),pt=v(Zt),Et=o(),y=e("table"),I=e("thead"),I.innerHTML=xt,Tt=o(),U=e("tbody"),z=e("tr"),H=e("td"),bt=v(te),yt=v(" m"),Dt=o(),k=e("td"),gt=v(ee),qt=v(" kg"),zt=o(),D=e("table"),O=e("thead"),O.innerHTML=ae,Ht=o(),X=e("tbody"),E=e("tr"),j=e("td"),kt=v(le),At=o(),C=e("td"),Lt=v(se),Mt=o(),P=e("td"),St=v(re),Bt=o(),g=e("table"),R=e("thead"),R.innerHTML=de,wt=o(),Z=e("tbody"),i=e("tr"),Y=e("td"),It=v(oe),Ot=o(),V=e("td"),jt=v(ne),Ct=o(),N=e("td"),Pt=v(he),Rt=o(),F=e("td"),Yt=v(ve),Vt=o(),G=e("td"),Nt=v(ce),Ft=o(),W=e("td"),Gt=v(ie),this.h()},l(T){je("svelte-1i3wmwf",document.head).forEach(l),p=n(T),d=a(T,"DIV",{});var f=s(d);u=a(f,"H1",{class:!0});var A=s(u);L=c(A,"#"),ot=c(A,J),nt=n(A),ht=c(A,Jt),vt=n(A),ct=c(A,Kt),A.forEach(l),it=n(f),M=a(f,"DIV",{class:!0});var _e=s(M);S=a(_e,"IMG",{src:!0,alt:!0}),_e.forEach(l),_t=n(f),b=a(f,"TABLE",{class:!0});var tt=s(b);B=a(tt,"THEAD",{"data-svelte-h":!0}),dt(B)!=="svelte-hkzgjc"&&(B.innerHTML=Ut),ut=n(tt),K=a(tt,"TBODY",{});var ue=s(K);Q=a(ue,"TR",{});var fe=s(Q);w=a(fe,"TD",{class:!0});var me=s(w);q=a(me,"DIV",{});var et=s(q);ft=c(et,Xt),mt=n(et),pt=c(et,Zt),et.forEach(l),me.forEach(l),fe.forEach(l),ue.forEach(l),tt.forEach(l),Et=n(f),y=a(f,"TABLE",{class:!0});var at=s(y);I=a(at,"THEAD",{"data-svelte-h":!0}),dt(I)!=="svelte-h6l440"&&(I.innerHTML=xt),Tt=n(at),U=a(at,"TBODY",{});var pe=s(U);z=a(pe,"TR",{});var lt=s(z);H=a(lt,"TD",{class:!0});var Wt=s(H);bt=c(Wt,te),yt=c(Wt," m"),Wt.forEach(l),Dt=n(lt),k=a(lt,"TD",{class:!0});var $t=s(k);gt=c($t,ee),qt=c($t," kg"),$t.forEach(l),lt.forEach(l),pe.forEach(l),at.forEach(l),zt=n(f),D=a(f,"TABLE",{class:!0});var st=s(D);O=a(st,"THEAD",{"data-svelte-h":!0}),dt(O)!=="svelte-pqbeq9"&&(O.innerHTML=ae),Ht=n(st),X=a(st,"TBODY",{});var Ee=s(X);E=a(Ee,"TR",{});var $=s(E);j=a($,"TD",{class:!0});var Te=s(j);kt=c(Te,le),Te.forEach(l),At=n($),C=a($,"TD",{class:!0});var be=s(C);Lt=c(be,se),be.forEach(l),Mt=n($),P=a($,"TD",{class:!0});var ye=s(P);St=c(ye,re),ye.forEach(l),$.forEach(l),Ee.forEach(l),st.forEach(l),Bt=n(f),g=a(f,"TABLE",{class:!0});var rt=s(g);R=a(rt,"THEAD",{"data-svelte-h":!0}),dt(R)!=="svelte-1ikhtwz"&&(R.innerHTML=de),wt=n(rt),Z=a(rt,"TBODY",{});var De=s(Z);i=a(De,"TR",{});var m=s(i);Y=a(m,"TD",{class:!0});var ge=s(Y);It=c(ge,oe),ge.forEach(l),Ot=n(m),V=a(m,"TD",{class:!0});var qe=s(V);jt=c(qe,ne),qe.forEach(l),Ct=n(m),N=a(m,"TD",{class:!0});var ze=s(N);Pt=c(ze,he),ze.forEach(l),Rt=n(m),F=a(m,"TD",{class:!0});var He=s(F);Yt=c(He,ve),He.forEach(l),Vt=n(m),G=a(m,"TD",{class:!0});var ke=s(G);Nt=c(ke,ce),ke.forEach(l),Ft=n(m),W=a(m,"TD",{class:!0});var Ae=s(W);Gt=c(Ae,ie),Ae.forEach(l),m.forEach(l),De.forEach(l),rt.forEach(l),f.forEach(l),this.h()},h(){h(u,"class","svelte-19adqrz"),we(S.src,Qt="/PokemonHome/"+r[0].pokemonimagefilename)||h(S,"src",Qt),h(S,"alt",r[0].name),h(M,"class","image svelte-19adqrz"),h(w,"class","svelte-19adqrz"),h(b,"class","svelte-19adqrz"),h(H,"class","svelte-19adqrz"),h(k,"class","svelte-19adqrz"),h(y,"class","svelte-19adqrz"),h(j,"class","svelte-19adqrz"),h(C,"class","svelte-19adqrz"),h(P,"class","svelte-19adqrz"),h(D,"class","svelte-19adqrz"),h(Y,"class","svelte-19adqrz"),h(V,"class","svelte-19adqrz"),h(N,"class","svelte-19adqrz"),h(F,"class","svelte-19adqrz"),h(G,"class","svelte-19adqrz"),h(W,"class","svelte-19adqrz"),h(g,"class","svelte-19adqrz")},m(T,x){Me(T,p,x),Me(T,d,x),t(d,u),t(u,L),t(u,ot),t(u,nt),t(u,ht),t(u,vt),t(u,ct),t(d,it),t(d,M),t(M,S),t(d,_t),t(d,b),t(b,B),t(b,ut),t(b,K),t(K,Q),t(Q,w),t(w,q),t(q,ft),t(q,mt),t(q,pt),t(d,Et),t(d,y),t(y,I),t(y,Tt),t(y,U),t(U,z),t(z,H),t(H,bt),t(H,yt),t(z,Dt),t(z,k),t(k,gt),t(k,qt),t(d,zt),t(d,D),t(D,O),t(D,Ht),t(D,X),t(X,E),t(E,j),t(j,kt),t(E,At),t(E,C),t(C,Lt),t(E,Mt),t(E,P),t(P,St),t(d,Bt),t(d,g),t(g,R),t(g,wt),t(g,Z),t(Z,i),t(i,Y),t(Y,It),t(i,Ot),t(i,V),t(V,jt),t(i,Ct),t(i,N),t(N,Pt),t(i,Rt),t(i,F),t(F,Yt),t(i,Vt),t(i,G),t(G,Nt),t(i,Ft),t(i,W),t(W,Gt)},p(T,[x]){x&1&&_!==(_=T[0].name)&&(document.title=_)},i:Le,o:Le,d(T){T&&(l(p),l(d))}}}let Ye=0;function Ve(r,_,p){let{data:d}=_,L=d.pokemonInfo[Ye];return console.log("+page.svelte:",L),r.$$set=J=>{"data"in J&&p(1,d=J.data)},[L,d]}class $e extends Ie{constructor(_){super(),Oe(this,_,Ve,Re,Be,{data:1})}}export{$e as component,We as universal};
