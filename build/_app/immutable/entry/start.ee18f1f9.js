import{n as be,s as it,o as ve,t as Ee}from"../chunks/scheduler.e8b33e29.js";import{R as Fe,H as ae}from"../chunks/control.f5b05b5f.js";function st(e,n){return e==="/"||n==="ignore"?e:n==="never"?e.endsWith("/")?e.slice(0,-1):e:n==="always"&&!e.endsWith("/")?e+"/":e}function ct(e){return e.split("%25").map(decodeURI).join("%25")}function lt(e){for(const n in e)e[n]=decodeURIComponent(e[n]);return e}const ft=["href","pathname","search","searchParams","toString","toJSON"];function ut(e,n){const s=new URL(e);for(const o of ft)Object.defineProperty(s,o,{get(){return n(),e[o]},enumerable:!0,configurable:!0});return dt(s),s}function dt(e){Object.defineProperty(e,"hash",{get(){throw new Error("Cannot access event.url.hash. Consider using `$page.url.hash` inside a component instead")}})}const pt="/__data.json";function ht(e){return e.replace(/\/$/,"")+pt}function gt(...e){let n=5381;for(const s of e)if(typeof s=="string"){let o=s.length;for(;o;)n=n*33^s.charCodeAt(--o)}else if(ArrayBuffer.isView(s)){const o=new Uint8Array(s.buffer,s.byteOffset,s.byteLength);let f=o.length;for(;f;)n=n*33^o[--f]}else throw new TypeError("value must be a string or TypedArray");return(n>>>0).toString(36)}const Ze=window.fetch;window.fetch=(e,n)=>((e instanceof Request?e.method:(n==null?void 0:n.method)||"GET")!=="GET"&&oe.delete(Ie(e)),Ze(e,n));const oe=new Map;function mt(e,n){const s=Ie(e,n),o=document.querySelector(s);if(o!=null&&o.textContent){const{body:f,...u}=JSON.parse(o.textContent),_=o.getAttribute("data-ttl");return _&&oe.set(s,{body:f,init:u,ttl:1e3*Number(_)}),Promise.resolve(new Response(f,u))}return window.fetch(e,n)}function _t(e,n,s){if(oe.size>0){const o=Ie(e,s),f=oe.get(o);if(f){if(performance.now()<f.ttl&&["default","force-cache","only-if-cached",void 0].includes(s==null?void 0:s.cache))return new Response(f.body,f.init);oe.delete(o)}}return window.fetch(n,s)}function Ie(e,n){let o=`script[data-sveltekit-fetched][data-url=${JSON.stringify(e instanceof Request?e.url:e)}]`;if(n!=null&&n.headers||n!=null&&n.body){const f=[];n.headers&&f.push([...new Headers(n.headers)].join(",")),n.body&&(typeof n.body=="string"||ArrayBuffer.isView(n.body))&&f.push(n.body),o+=`[data-hash="${gt(...f)}"]`}return o}const wt=/^(\[)?(\.\.\.)?(\w+)(?:=(\w+))?(\])?$/;function yt(e){const n=[];return{pattern:e==="/"?/^\/$/:new RegExp(`^${vt(e).map(o=>{const f=/^\[\.\.\.(\w+)(?:=(\w+))?\]$/.exec(o);if(f)return n.push({name:f[1],matcher:f[2],optional:!1,rest:!0,chained:!0}),"(?:/(.*))?";const u=/^\[\[(\w+)(?:=(\w+))?\]\]$/.exec(o);if(u)return n.push({name:u[1],matcher:u[2],optional:!0,rest:!1,chained:!0}),"(?:/([^/]+))?";if(!o)return;const _=o.split(/\[(.+?)\](?!\])/);return"/"+_.map((h,p)=>{if(p%2){if(h.startsWith("x+"))return ke(String.fromCharCode(parseInt(h.slice(2),16)));if(h.startsWith("u+"))return ke(String.fromCharCode(...h.slice(2).split("-").map(P=>parseInt(P,16))));const m=wt.exec(h);if(!m)throw new Error(`Invalid param: ${h}. Params and matcher names can only have underscores and alphanumeric characters.`);const[,x,T,S,j]=m;return n.push({name:S,matcher:j,optional:!!x,rest:!!T,chained:T?p===1&&_[0]==="":!1}),T?"(.*?)":x?"([^/]*)?":"([^/]+?)"}return ke(h)}).join("")}).join("")}/?$`),params:n}}function bt(e){return!/^\([^)]+\)$/.test(e)}function vt(e){return e.slice(1).split("/").filter(bt)}function Et(e,n,s){const o={},f=e.slice(1),u=f.filter(i=>i!==void 0);let _=0;for(let i=0;i<n.length;i+=1){const h=n[i];let p=f[i-_];if(h.chained&&h.rest&&_&&(p=f.slice(i-_,i+1).filter(m=>m).join("/"),_=0),p===void 0){h.rest&&(o[h.name]="");continue}if(!h.matcher||s[h.matcher](p)){o[h.name]=p;const m=n[i+1],x=f[i+1];m&&!m.rest&&m.optional&&x&&h.chained&&(_=0),!m&&!x&&Object.keys(o).length===u.length&&(_=0);continue}if(h.optional&&h.chained){_++;continue}return}if(!_)return o}function ke(e){return e.normalize().replace(/[[\]]/g,"\\$&").replace(/%/g,"%25").replace(/\//g,"%2[Ff]").replace(/\?/g,"%3[Ff]").replace(/#/g,"%23").replace(/[.*+?^${}()|\\]/g,"\\$&")}function kt({nodes:e,server_loads:n,dictionary:s,matchers:o}){const f=new Set(n);return Object.entries(s).map(([i,[h,p,m]])=>{const{pattern:x,params:T}=yt(i),S={id:i,exec:j=>{const P=x.exec(j);if(P)return Et(P,T,o)},errors:[1,...m||[]].map(j=>e[j]),layouts:[0,...p||[]].map(_),leaf:u(h)};return S.errors.length=S.layouts.length=Math.max(S.errors.length,S.layouts.length),S});function u(i){const h=i<0;return h&&(i=~i),[h,e[i]]}function _(i){return i===void 0?i:[f.has(i),e[i]]}}function Qe(e){try{return JSON.parse(sessionStorage[e])}catch{}}function Ge(e,n){const s=JSON.stringify(n);try{sessionStorage[e]=s}catch{}}const Y=[];function Le(e,n=be){let s;const o=new Set;function f(i){if(it(e,i)&&(e=i,s)){const h=!Y.length;for(const p of o)p[1](),Y.push(p,e);if(h){for(let p=0;p<Y.length;p+=2)Y[p][0](Y[p+1]);Y.length=0}}}function u(i){f(i(e))}function _(i,h=be){const p=[i,h];return o.add(p),o.size===1&&(s=n(f,u)||be),i(e),()=>{o.delete(p),o.size===0&&s&&(s(),s=null)}}return{set:f,update:u,subscribe:_}}var Ye;const G=((Ye=globalThis.__sveltekit_yayhw8)==null?void 0:Ye.base)??"";var Xe;const St=((Xe=globalThis.__sveltekit_yayhw8)==null?void 0:Xe.assets)??G,Rt="1698627174831",et="sveltekit:snapshot",tt="sveltekit:scroll",V="sveltekit:index",pe={tap:1,hover:2,viewport:3,eager:4,off:-1};function He(e){let n=e.baseURI;if(!n){const s=e.getElementsByTagName("base");n=s.length?s[0].href:e.URL}return n}function re(){return{x:pageXOffset,y:pageYOffset}}function X(e,n){return e.getAttribute(`data-sveltekit-${n}`)}const ze={...pe,"":pe.hover};function nt(e){let n=e.assignedSlot??e.parentNode;return(n==null?void 0:n.nodeType)===11&&(n=n.host),n}function Be(e,n){for(;e&&e!==n;){if(e.nodeName.toUpperCase()==="A"&&e.hasAttribute("href"))return e;e=nt(e)}}function Se(e,n){let s;try{s=new URL(e instanceof SVGAElement?e.href.baseVal:e.href,document.baseURI)}catch{}const o=e instanceof SVGAElement?e.target.baseVal:e.target,f=!s||!!o||de(s,n)||(e.getAttribute("rel")||"").split(/\s+/).includes("external"),u=(s==null?void 0:s.origin)===location.origin&&e.hasAttribute("download");return{url:s,external:f,target:o,download:u}}function ue(e){let n=null,s=null,o=null,f=null,u=null,_=null,i=e;for(;i&&i!==document.documentElement;)o===null&&(o=X(i,"preload-code")),f===null&&(f=X(i,"preload-data")),n===null&&(n=X(i,"keepfocus")),s===null&&(s=X(i,"noscroll")),u===null&&(u=X(i,"reload")),_===null&&(_=X(i,"replacestate")),i=nt(i);function h(p){switch(p){case"":case"true":return!0;case"off":case"false":return!1;default:return null}}return{preload_code:ze[o??"off"],preload_data:ze[f??"off"],keep_focus:h(n),noscroll:h(s),reload:h(u),replace_state:h(_)}}function Ke(e){const n=Le(e);let s=!0;function o(){s=!0,n.update(_=>_)}function f(_){s=!1,n.set(_)}function u(_){let i;return n.subscribe(h=>{(i===void 0||s&&h!==i)&&_(i=h)})}return{notify:o,set:f,subscribe:u}}function At(){const{set:e,subscribe:n}=Le(!1);let s;async function o(){clearTimeout(s);try{const f=await fetch(`${St}/_app/version.json`,{headers:{pragma:"no-cache","cache-control":"no-cache"}});if(!f.ok)return!1;const _=(await f.json()).version!==Rt;return _&&(e(!0),clearTimeout(s)),_}catch{return!1}}return{subscribe:n,check:o}}function de(e,n){return e.origin!==location.origin||!e.pathname.startsWith(n)}const It=-1,Lt=-2,Ut=-3,Ot=-4,Pt=-5,xt=-6;function Tt(e,n){if(typeof e=="number")return f(e,!0);if(!Array.isArray(e)||e.length===0)throw new Error("Invalid input");const s=e,o=Array(s.length);function f(u,_=!1){if(u===It)return;if(u===Ut)return NaN;if(u===Ot)return 1/0;if(u===Pt)return-1/0;if(u===xt)return-0;if(_)throw new Error("Invalid input");if(u in o)return o[u];const i=s[u];if(!i||typeof i!="object")o[u]=i;else if(Array.isArray(i))if(typeof i[0]=="string"){const h=i[0],p=n==null?void 0:n[h];if(p)return o[u]=p(f(i[1]));switch(h){case"Date":o[u]=new Date(i[1]);break;case"Set":const m=new Set;o[u]=m;for(let S=1;S<i.length;S+=1)m.add(f(i[S]));break;case"Map":const x=new Map;o[u]=x;for(let S=1;S<i.length;S+=2)x.set(f(i[S]),f(i[S+1]));break;case"RegExp":o[u]=new RegExp(i[1],i[2]);break;case"Object":o[u]=Object(i[1]);break;case"BigInt":o[u]=BigInt(i[1]);break;case"null":const T=Object.create(null);o[u]=T;for(let S=1;S<i.length;S+=2)T[i[S]]=f(i[S+1]);break;default:throw new Error(`Unknown type ${h}`)}}else{const h=new Array(i.length);o[u]=h;for(let p=0;p<i.length;p+=1){const m=i[p];m!==Lt&&(h[p]=f(m))}}else{const h={};o[u]=h;for(const p in i){const m=i[p];h[p]=f(m)}}return o[u]}return f(0)}function jt(e){return e.filter(n=>n!=null)}const at=new Set(["load","prerender","csr","ssr","trailingSlash","config"]);[...at];const Nt=new Set([...at]);[...Nt];async function $t(e){var n;for(const s in e)if(typeof((n=e[s])==null?void 0:n.then)=="function")return Object.fromEntries(await Promise.all(Object.entries(e).map(async([o,f])=>[o,await f])));return e}const Ct="x-sveltekit-invalidated",Dt="x-sveltekit-trailing-slash";function Vt(e){e.client}const F={url:Ke({}),page:Ke({}),navigating:Le(null),updated:At()},K=Qe(tt)??{},ne=Qe(et)??{};function Re(e){K[e]=re()}function qt(e,n){var Ve;const s=kt(e),o=e.nodes[0],f=e.nodes[1];o(),f();const u=document.documentElement,_=[],i=[];let h=null;const p={before_navigate:[],on_navigate:[],after_navigate:[]};let m={branch:[],error:null,url:null},x=!1,T=!1,S=!0,j=!1,P=!1,H=!1,z=!1,q,C=(Ve=history.state)==null?void 0:Ve[V];C||(C=Date.now(),history.replaceState({...history.state,[V]:C},"",location.href));const he=K[C];he&&(history.scrollRestoration="manual",scrollTo(he.x,he.y));let M,J,Z;async function Ue(){if(Z=Z||Promise.resolve(),await Z,!Z)return;Z=null;const t=new URL(location.href),l=ee(t,!0);h=null;const a=J={},c=l&&await _e(l);if(a===J&&c){if(c.type==="redirect")return ie(new URL(c.location,t).href,{},1,a);c.props.page!==void 0&&(M=c.props.page),q.$set(c.props)}}function Oe(t){i.some(l=>l==null?void 0:l.snapshot)&&(ne[t]=i.map(l=>{var a;return(a=l==null?void 0:l.snapshot)==null?void 0:a.capture()}))}function Pe(t){var l;(l=ne[t])==null||l.forEach((a,c)=>{var r,d;(d=(r=i[c])==null?void 0:r.snapshot)==null||d.restore(a)})}function xe(){Re(C),Ge(tt,K),Oe(C),Ge(et,ne)}async function ie(t,{noScroll:l=!1,replaceState:a=!1,keepFocus:c=!1,state:r={},invalidateAll:d=!1},g,v){return typeof t=="string"&&(t=new URL(t,He(document))),fe({url:t,scroll:l?re():null,keepfocus:c,redirect_count:g,details:{state:r,replaceState:a},nav_token:v,accepted:()=>{d&&(z=!0)},blocked:()=>{},type:"goto"})}async function Te(t){return h={id:t.id,promise:_e(t).then(l=>(l.type==="loaded"&&l.state.error&&(h=null),l))},h.promise}async function se(...t){const a=s.filter(c=>t.some(r=>c.exec(r))).map(c=>Promise.all([...c.layouts,c.leaf].map(r=>r==null?void 0:r[1]())));await Promise.all(a)}function je(t){var c;m=t.state;const l=document.querySelector("style[data-sveltekit]");l&&l.remove(),M=t.props.page,q=new e.root({target:n,props:{...t.props,stores:F,components:i},hydrate:!0}),Pe(C);const a={from:null,to:{params:m.params,route:{id:((c=m.route)==null?void 0:c.id)??null},url:new URL(location.href)},willUnload:!1,type:"enter",complete:Promise.resolve()};p.after_navigate.forEach(r=>r(a)),T=!0}async function Q({url:t,params:l,branch:a,status:c,error:r,route:d,form:g}){let v="never";for(const y of a)(y==null?void 0:y.slash)!==void 0&&(v=y.slash);t.pathname=st(t.pathname,v),t.search=t.search;const E={type:"loaded",state:{url:t,params:l,branch:a,error:r,route:d},props:{constructors:jt(a).map(y=>y.node.component)}};g!==void 0&&(E.props.form=g);let b={},L=!M,A=0;for(let y=0;y<Math.max(a.length,m.branch.length);y+=1){const w=a[y],O=m.branch[y];(w==null?void 0:w.data)!==(O==null?void 0:O.data)&&(L=!0),w&&(b={...b,...w.data},L&&(E.props[`data_${A}`]=b),A+=1)}return(!m.url||t.href!==m.url.href||m.error!==r||g!==void 0&&g!==M.form||L)&&(E.props.page={error:r,params:l,route:{id:(d==null?void 0:d.id)??null},status:c,url:new URL(t),form:g??null,data:L?b:M.data}),E}async function ge({loader:t,parent:l,url:a,params:c,route:r,server_data_node:d}){var b,L,A;let g=null;const v={dependencies:new Set,params:new Set,parent:!1,route:!1,url:!1},E=await t();if((b=E.universal)!=null&&b.load){let U=function(...w){for(const O of w){const{href:N}=new URL(O,a);v.dependencies.add(N)}};const y={route:new Proxy(r,{get:(w,O)=>(v.route=!0,w[O])}),params:new Proxy(c,{get:(w,O)=>(v.params.add(O),w[O])}),data:(d==null?void 0:d.data)??null,url:ut(a,()=>{v.url=!0}),async fetch(w,O){let N;w instanceof Request?(N=w.url,O={body:w.method==="GET"||w.method==="HEAD"?void 0:await w.blob(),cache:w.cache,credentials:w.credentials,headers:w.headers,integrity:w.integrity,keepalive:w.keepalive,method:w.method,mode:w.mode,redirect:w.redirect,referrer:w.referrer,referrerPolicy:w.referrerPolicy,signal:w.signal,...O}):N=w;const D=new URL(N,a);return U(D.href),D.origin===a.origin&&(N=D.href.slice(a.origin.length)),T?_t(N,D.href,O):mt(N,O)},setHeaders:()=>{},depends:U,parent(){return v.parent=!0,l()}};g=await E.universal.load.call(null,y)??null,g=g?await $t(g):null}return{node:E,loader:t,server:d,universal:(L=E.universal)!=null&&L.load?{type:"data",data:g,uses:v}:null,data:g??(d==null?void 0:d.data)??null,slash:((A=E.universal)==null?void 0:A.trailingSlash)??(d==null?void 0:d.slash)}}function Ne(t,l,a,c,r){if(z)return!0;if(!c)return!1;if(c.parent&&t||c.route&&l||c.url&&a)return!0;for(const d of c.params)if(r[d]!==m.params[d])return!0;for(const d of c.dependencies)if(_.some(g=>g(new URL(d))))return!0;return!1}function me(t,l){return(t==null?void 0:t.type)==="data"?t:(t==null?void 0:t.type)==="skip"?l??null:null}async function _e({id:t,invalidating:l,url:a,params:c,route:r}){if((h==null?void 0:h.id)===t)return h.promise;const{errors:d,layouts:g,leaf:v}=r,E=[...g,v];d.forEach(k=>k==null?void 0:k().catch(()=>{})),E.forEach(k=>k==null?void 0:k[1]().catch(()=>{}));let b=null;const L=m.url?t!==m.url.pathname+m.url.search:!1,A=m.route?r.id!==m.route.id:!1;let U=!1;const y=E.map((k,I)=>{var B;const R=m.branch[I],$=!!(k!=null&&k[0])&&((R==null?void 0:R.loader)!==k[1]||Ne(U,A,L,(B=R.server)==null?void 0:B.uses,c));return $&&(U=!0),$});if(y.some(Boolean)){try{b=await Je(a,y)}catch(k){return ce({status:k instanceof ae?k.status:500,error:await te(k,{url:a,params:c,route:{id:r.id}}),url:a,route:r})}if(b.type==="redirect")return b}const w=b==null?void 0:b.nodes;let O=!1;const N=E.map(async(k,I)=>{var we;if(!k)return;const R=m.branch[I],$=w==null?void 0:w[I];if((!$||$.type==="skip")&&k[1]===(R==null?void 0:R.loader)&&!Ne(O,A,L,(we=R.universal)==null?void 0:we.uses,c))return R;if(O=!0,($==null?void 0:$.type)==="error")throw $;return ge({loader:k[1],url:a,params:c,route:r,parent:async()=>{var Me;const qe={};for(let ye=0;ye<I;ye+=1)Object.assign(qe,(Me=await N[ye])==null?void 0:Me.data);return qe},server_data_node:me($===void 0&&k[0]?{type:"skip"}:$??null,k[0]?R==null?void 0:R.server:void 0)})});for(const k of N)k.catch(()=>{});const D=[];for(let k=0;k<E.length;k+=1)if(E[k])try{D.push(await N[k])}catch(I){if(I instanceof Fe)return{type:"redirect",location:I.location};let R=500,$;if(w!=null&&w.includes(I))R=I.status??R,$=I.error;else if(I instanceof ae)R=I.status,$=I.body;else{if(await F.updated.check())return await W(a);$=await te(I,{params:c,url:a,route:{id:r.id}})}const B=await $e(k,D,d);return B?await Q({url:a,params:c,branch:D.slice(0,B.idx).concat(B.node),status:R,error:$,route:r}):await De(a,{id:r.id},$,R)}else D.push(void 0);return await Q({url:a,params:c,branch:D,status:200,error:null,route:r,form:l?void 0:null})}async function $e(t,l,a){for(;t--;)if(a[t]){let c=t;for(;!l[c];)c-=1;try{return{idx:c+1,node:{node:await a[t](),loader:a[t],data:{},server:null,universal:null}}}catch{continue}}}async function ce({status:t,error:l,url:a,route:c}){const r={};let d=null;if(e.server_loads[0]===0)try{const b=await Je(a,[!0]);if(b.type!=="data"||b.nodes[0]&&b.nodes[0].type!=="data")throw 0;d=b.nodes[0]??null}catch{(a.origin!==location.origin||a.pathname!==location.pathname||x)&&await W(a)}const v=await ge({loader:o,url:a,params:r,route:c,parent:()=>Promise.resolve({}),server_data_node:me(d)}),E={node:await f(),loader:f,universal:null,server:null,data:null};return await Q({url:a,params:r,branch:[v,E],status:t,error:l,route:null})}function ee(t,l){if(de(t,G))return;const a=le(t);for(const c of s){const r=c.exec(a);if(r)return{id:t.pathname+t.search,invalidating:l,route:c,params:lt(r),url:t}}}function le(t){return ct(t.pathname.slice(G.length)||"/")}function Ce({url:t,type:l,intent:a,delta:c}){let r=!1;const d=We(m,a,t,l);c!==void 0&&(d.navigation.delta=c);const g={...d.navigation,cancel:()=>{r=!0,d.reject(new Error("navigation was cancelled"))}};return P||p.before_navigate.forEach(v=>v(g)),r?null:d}async function fe({url:t,scroll:l,keepfocus:a,redirect_count:c,details:r,type:d,delta:g,nav_token:v={},accepted:E,blocked:b}){var N,D,k;const L=ee(t,!1),A=Ce({url:t,type:d,delta:g,intent:L});if(!A){b();return}const U=C;E(),P=!0,T&&F.navigating.set(A.navigation),J=v;let y=L&&await _e(L);if(!y){if(de(t,G))return await W(t);y=await De(t,{id:null},await te(new Error(`Not found: ${t.pathname}`),{url:t,params:{},route:{id:null}}),404)}if(t=(L==null?void 0:L.url)||t,J!==v)return A.reject(new Error("navigation was aborted")),!1;if(y.type==="redirect")if(c>=20)y=await ce({status:500,error:await te(new Error("Redirect loop"),{url:t,params:{},route:{id:null}}),url:t,route:{id:null}});else return ie(new URL(y.location,t).href,{},c+1,v),!1;else((N=y.props.page)==null?void 0:N.status)>=400&&await F.updated.check()&&await W(t);if(_.length=0,z=!1,j=!0,Re(U),Oe(U),(D=y.props.page)!=null&&D.url&&y.props.page.url.pathname!==t.pathname&&(t.pathname=(k=y.props.page)==null?void 0:k.url.pathname),r){const I=r.replaceState?0:1;if(r.state[V]=C+=I,history[r.replaceState?"replaceState":"pushState"](r.state,"",t),!r.replaceState){let R=C+1;for(;ne[R]||K[R];)delete ne[R],delete K[R],R+=1}}if(h=null,T){m=y.state,y.props.page&&(y.props.page.url=t);const I=(await Promise.all(p.on_navigate.map(R=>R(A.navigation)))).filter(R=>typeof R=="function");if(I.length>0){let R=function(){p.after_navigate=p.after_navigate.filter($=>!I.includes($))};I.push(R),p.after_navigate.push(...I)}q.$set(y.props)}else je(y);const{activeElement:w}=document;if(await Ee(),S){const I=t.hash&&document.getElementById(decodeURIComponent(t.hash.slice(1)));l?scrollTo(l.x,l.y):I?I.scrollIntoView():scrollTo(0,0)}const O=document.activeElement!==w&&document.activeElement!==document.body;!a&&!O&&Ae(),S=!0,y.props.page&&(M=y.props.page),P=!1,d==="popstate"&&Pe(C),A.fulfil(void 0),p.after_navigate.forEach(I=>I(A.navigation)),F.navigating.set(null),j=!1}async function De(t,l,a,c){return t.origin===location.origin&&t.pathname===location.pathname&&!x?await ce({status:c,error:a,url:t,route:l}):await W(t)}function W(t){return location.href=t.href,new Promise(()=>{})}function ot(){let t;u.addEventListener("mousemove",d=>{const g=d.target;clearTimeout(t),t=setTimeout(()=>{c(g,2)},20)});function l(d){c(d.composedPath()[0],1)}u.addEventListener("mousedown",l),u.addEventListener("touchstart",l,{passive:!0});const a=new IntersectionObserver(d=>{for(const g of d)g.isIntersecting&&(se(le(new URL(g.target.href))),a.unobserve(g.target))},{threshold:0});function c(d,g){const v=Be(d,u);if(!v)return;const{url:E,external:b,download:L}=Se(v,G);if(b||L)return;const A=ue(v);if(!A.reload)if(g<=A.preload_data){const U=ee(E,!1);U&&Te(U)}else g<=A.preload_code&&se(le(E))}function r(){a.disconnect();for(const d of u.querySelectorAll("a")){const{url:g,external:v,download:E}=Se(d,G);if(v||E)continue;const b=ue(d);b.reload||(b.preload_code===pe.viewport&&a.observe(d),b.preload_code===pe.eager&&se(le(g)))}}p.after_navigate.push(r),r()}function te(t,l){return t instanceof ae?t.body:e.hooks.handleError({error:t,event:l})??{message:l.route.id!=null?"Internal Error":"Not Found"}}return{after_navigate:t=>{ve(()=>(p.after_navigate.push(t),()=>{const l=p.after_navigate.indexOf(t);p.after_navigate.splice(l,1)}))},before_navigate:t=>{ve(()=>(p.before_navigate.push(t),()=>{const l=p.before_navigate.indexOf(t);p.before_navigate.splice(l,1)}))},on_navigate:t=>{ve(()=>(p.on_navigate.push(t),()=>{const l=p.on_navigate.indexOf(t);p.on_navigate.splice(l,1)}))},disable_scroll_handling:()=>{(j||!T)&&(S=!1)},goto:(t,l={})=>ie(t,l,0),invalidate:t=>{if(typeof t=="function")_.push(t);else{const{href:l}=new URL(t,location.href);_.push(a=>a.href===l)}return Ue()},invalidate_all:()=>(z=!0,Ue()),preload_data:async t=>{const l=new URL(t,He(document)),a=ee(l,!1);if(!a)throw new Error(`Attempted to preload a URL that does not belong to this app: ${l}`);await Te(a)},preload_code:se,apply_action:async t=>{if(t.type==="error"){const l=new URL(location.href),{branch:a,route:c}=m;if(!c)return;const r=await $e(m.branch.length,a,c.errors);if(r){const d=await Q({url:l,params:m.params,branch:a.slice(0,r.idx).concat(r.node),status:t.status??500,error:t.error,route:c});m=d.state,q.$set(d.props),Ee().then(Ae)}}else t.type==="redirect"?ie(t.location,{invalidateAll:!0},0):(q.$set({form:null,page:{...M,form:t.data,status:t.status}}),await Ee(),q.$set({form:t.data}),t.type==="success"&&Ae())},_start_router:()=>{var l;history.scrollRestoration="manual",addEventListener("beforeunload",a=>{let c=!1;if(xe(),!P){const r=We(m,void 0,null,"leave"),d={...r.navigation,cancel:()=>{c=!0,r.reject(new Error("navigation was cancelled"))}};p.before_navigate.forEach(g=>g(d))}c?(a.preventDefault(),a.returnValue=""):history.scrollRestoration="auto"}),addEventListener("visibilitychange",()=>{document.visibilityState==="hidden"&&xe()}),(l=navigator.connection)!=null&&l.saveData||ot(),u.addEventListener("click",a=>{var U;if(a.button||a.which!==1||a.metaKey||a.ctrlKey||a.shiftKey||a.altKey||a.defaultPrevented)return;const c=Be(a.composedPath()[0],u);if(!c)return;const{url:r,external:d,target:g,download:v}=Se(c,G);if(!r)return;if(g==="_parent"||g==="_top"){if(window.parent!==window)return}else if(g&&g!=="_self")return;const E=ue(c);if(!(c instanceof SVGAElement)&&r.protocol!==location.protocol&&!(r.protocol==="https:"||r.protocol==="http:")||v)return;if(d||E.reload){Ce({url:r,type:"link"})?P=!0:a.preventDefault();return}const[L,A]=r.href.split("#");if(A!==void 0&&L===location.href.split("#")[0]){if(m.url.hash===r.hash){a.preventDefault(),(U=c.ownerDocument.getElementById(A))==null||U.scrollIntoView();return}if(H=!0,Re(C),t(r),!E.replace_state)return;H=!1,a.preventDefault()}fe({url:r,scroll:E.noscroll?re():null,keepfocus:E.keep_focus??!1,redirect_count:0,details:{state:{},replaceState:E.replace_state??r.href===location.href},accepted:()=>a.preventDefault(),blocked:()=>a.preventDefault(),type:"link"})}),u.addEventListener("submit",a=>{if(a.defaultPrevented)return;const c=HTMLFormElement.prototype.cloneNode.call(a.target),r=a.submitter;if(((r==null?void 0:r.formMethod)||c.method)!=="get")return;const g=new URL((r==null?void 0:r.hasAttribute("formaction"))&&(r==null?void 0:r.formAction)||c.action);if(de(g,G))return;const v=a.target,{keep_focus:E,noscroll:b,reload:L,replace_state:A}=ue(v);if(L)return;a.preventDefault(),a.stopPropagation();const U=new FormData(v),y=r==null?void 0:r.getAttribute("name");y&&U.append(y,(r==null?void 0:r.getAttribute("value"))??""),g.search=new URLSearchParams(U).toString(),fe({url:g,scroll:b?re():null,keepfocus:E??!1,redirect_count:0,details:{state:{},replaceState:A??g.href===location.href},nav_token:{},accepted:()=>{},blocked:()=>{},type:"form"})}),addEventListener("popstate",async a=>{var c;if(J={},(c=a.state)!=null&&c[V]){if(a.state[V]===C)return;const r=K[a.state[V]],d=new URL(location.href);if(m.url.href.split("#")[0]===location.href.split("#")[0]){t(d),K[C]=re(),C=a.state[V],scrollTo(r.x,r.y);return}const g=a.state[V]-C;await fe({url:d,scroll:r,keepfocus:!1,redirect_count:0,details:null,accepted:()=>{C=a.state[V]},blocked:()=>{history.go(-g)},type:"popstate",delta:g,nav_token:J})}else if(!H){const r=new URL(location.href);t(r)}}),addEventListener("hashchange",()=>{H&&(H=!1,history.replaceState({...history.state,[V]:++C},"",location.href))});for(const a of document.querySelectorAll("link"))a.rel==="icon"&&(a.href=a.href);addEventListener("pageshow",a=>{a.persisted&&F.navigating.set(null)});function t(a){m.url=a,F.page.set({...M,url:a}),F.page.notify()}},_hydrate:async({status:t=200,error:l,node_ids:a,params:c,route:r,data:d,form:g})=>{x=!0;const v=new URL(location.href);({params:c={},route:r={id:null}}=ee(v,!1)||{});let E;try{const b=a.map(async(U,y)=>{const w=d[y];return w!=null&&w.uses&&(w.uses=rt(w.uses)),ge({loader:e.nodes[U],url:v,params:c,route:r,parent:async()=>{const O={};for(let N=0;N<y;N+=1)Object.assign(O,(await b[N]).data);return O},server_data_node:me(w)})}),L=await Promise.all(b),A=s.find(({id:U})=>U===r.id);if(A){const U=A.layouts;for(let y=0;y<U.length;y++)U[y]||L.splice(y,0,void 0)}E=await Q({url:v,params:c,branch:L,status:t,error:l,form:g,route:A??null})}catch(b){if(b instanceof Fe){await W(new URL(b.location,location.href));return}E=await ce({status:b instanceof ae?b.status:500,error:await te(b,{url:v,params:c,route:r}),url:v,route:r})}je(E)}}}async function Je(e,n){const s=new URL(e);s.pathname=ht(e.pathname),e.pathname.endsWith("/")&&s.searchParams.append(Dt,"1"),s.searchParams.append(Ct,n.map(f=>f?"1":"0").join(""));const o=await Ze(s.href);if(!o.ok)throw new ae(o.status,await o.json());return new Promise(async f=>{var m;const u=new Map,_=o.body.getReader(),i=new TextDecoder;function h(x){return Tt(x,{Promise:T=>new Promise((S,j)=>{u.set(T,{fulfil:S,reject:j})})})}let p="";for(;;){const{done:x,value:T}=await _.read();if(x&&!p)break;for(p+=!T&&p?`
`:i.decode(T);;){const S=p.indexOf(`
`);if(S===-1)break;const j=JSON.parse(p.slice(0,S));if(p=p.slice(S+1),j.type==="redirect")return f(j);if(j.type==="data")(m=j.nodes)==null||m.forEach(P=>{(P==null?void 0:P.type)==="data"&&(P.uses=rt(P.uses),P.data=h(P.data))}),f(j);else if(j.type==="chunk"){const{id:P,data:H,error:z}=j,q=u.get(P);u.delete(P),z?q.reject(h(z)):q.fulfil(h(H))}}}})}function rt(e){return{dependencies:new Set((e==null?void 0:e.dependencies)??[]),params:new Set((e==null?void 0:e.params)??[]),parent:!!(e!=null&&e.parent),route:!!(e!=null&&e.route),url:!!(e!=null&&e.url)}}function Ae(){const e=document.querySelector("[autofocus]");if(e)e.focus();else{const n=document.body,s=n.getAttribute("tabindex");n.tabIndex=-1,n.focus({preventScroll:!0,focusVisible:!1}),s!==null?n.setAttribute("tabindex",s):n.removeAttribute("tabindex");const o=getSelection();if(o&&o.type!=="None"){const f=[];for(let u=0;u<o.rangeCount;u+=1)f.push(o.getRangeAt(u));setTimeout(()=>{if(o.rangeCount===f.length){for(let u=0;u<o.rangeCount;u+=1){const _=f[u],i=o.getRangeAt(u);if(_.commonAncestorContainer!==i.commonAncestorContainer||_.startContainer!==i.startContainer||_.endContainer!==i.endContainer||_.startOffset!==i.startOffset||_.endOffset!==i.endOffset)return}o.removeAllRanges()}})}}}function We(e,n,s,o){var h,p;let f,u;const _=new Promise((m,x)=>{f=m,u=x});return _.catch(()=>{}),{navigation:{from:{params:e.params,route:{id:((h=e.route)==null?void 0:h.id)??null},url:e.url},to:s&&{params:(n==null?void 0:n.params)??null,route:{id:((p=n==null?void 0:n.route)==null?void 0:p.id)??null},url:s},willUnload:!n,type:o,complete:_},fulfil:f,reject:u}}async function Gt(e,n,s){const o=qt(e,n);Vt({client:o}),s?await o._hydrate(s):o.goto(location.href,{replaceState:!0}),o._start_router()}export{Gt as start};
