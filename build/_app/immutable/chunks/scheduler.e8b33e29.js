function q(){}const M=t=>t;function k(t,n){for(const e in n)t[e]=n[e];return t}function x(t){return t()}function z(){return Object.create(null)}function w(t){t.forEach(x)}function F(t){return typeof t=="function"}function P(t,n){return t!=t?n==n:t!==n||t&&typeof t=="object"||typeof t=="function"}function S(t){return Object.keys(t).length===0}function U(t,n,e,o){if(t){const r=y(t,n,e,o);return t[0](r)}}function y(t,n,e,o){return t[1]&&o?k(e.ctx.slice(),t[1](o(n))):e.ctx}function A(t,n,e,o){if(t[2]&&o){const r=t[2](o(e));if(n.dirty===void 0)return r;if(typeof r=="object"){const a=[],f=Math.max(n.dirty.length,r.length);for(let u=0;u<f;u+=1)a[u]=n.dirty[u]|r[u];return a}return n.dirty|r}return n.dirty}function B(t,n,e,o,r,a){if(r){const f=y(n,e,o,a);t.p(f,r)}}function C(t){if(t.ctx.length>32){const n=[],e=t.ctx.length/32;for(let o=0;o<e;o++)n[o]=-1;return n}return-1}let i;function d(t){i=t}function m(){if(!i)throw new Error("Function called outside component initialization");return i}function D(t){m().$$.on_mount.push(t)}function G(t){m().$$.after_update.push(t)}const l=[],p=[];let s=[];const g=[],b=Promise.resolve();let _=!1;function j(){_||(_=!0,b.then(v))}function H(){return j(),b}function E(t){s.push(t)}const h=new Set;let c=0;function v(){if(c!==0)return;const t=i;do{try{for(;c<l.length;){const n=l[c];c++,d(n),O(n.$$)}}catch(n){throw l.length=0,c=0,n}for(d(null),l.length=0,c=0;p.length;)p.pop()();for(let n=0;n<s.length;n+=1){const e=s[n];h.has(e)||(h.add(e),e())}s.length=0}while(l.length);for(;g.length;)g.pop()();_=!1,h.clear(),d(t)}function O(t){if(t.fragment!==null){t.update(),w(t.before_update);const n=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,n),t.after_update.forEach(E)}}function I(t){const n=[],e=[];s.forEach(o=>t.indexOf(o)===-1?n.push(o):e.push(o)),e.forEach(o=>o()),s=n}export{G as a,p as b,U as c,A as d,E as e,F as f,C as g,z as h,M as i,v as j,S as k,I as l,i as m,q as n,D as o,d as p,x as q,w as r,P as s,H as t,B as u,l as v,j as w};
