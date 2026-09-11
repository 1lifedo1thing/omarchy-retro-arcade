// Prepare the generated colour-key atlas; no artwork is procedurally redrawn.
const sharp=require(process.env.CODEX_PRIMARY_RUNTIME_NODE_MODULES+'/sharp');
const fs=require('fs');
(async()=>{
 const palette=[[228,232,223],[179,203,146],[100,122,81],[255,155,54],[255,220,136]];
 const {data,info}=await sharp('docs/design/source-atlas.png').resize(192,128,{kernel:'nearest'}).ensureAlpha().raw().toBuffer({resolveWithObject:true});
 for(let i=0;i<data.length;i+=4){let [r,g,b]=data.subarray(i,i+3);if(r>150&&b>120&&g<110){data[i+3]=0;data[i]=data[i+1]=data[i+2]=0;continue;}let best=palette.reduce((a,c)=>Math.hypot(r-c[0],g-c[1],b-c[2])<Math.hypot(r-a[0],g-a[1],b-a[2])?c:a);for(let j=0;j<3;j++)data[i+j]=best[j];data[i+3]=255;}
 await sharp(data,{raw:info}).png().toFile('assets/orbit/atlas.png');
 await sharp('assets/orbit/atlas.png').extract({left:0,top:0,width:32,height:32}).resize(256,256,{kernel:'nearest'}).png().toFile('packaging/omarchy-invaders.png');
})();
