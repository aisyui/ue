# three-vrm

今回は`react-three-fiber`と`three-vrm-animation`を使います。

react-three-fiberはsceneなどを自動でやってくれて、コードもシンプルになります。

> `react-three-fiber`で書く場合に`.vrma`の動きがおかしくなりました。私の環境では`unity + vrm 1.0`でexportしたものを使うと正常に動きました。

```sh
$ npx create-react-app vrm-model --template typescript
$ npm i
$ npm run start
```

> src/pages/vrm-model.tsx

```ts
import * as THREE from 'three'
import React, { useState, useEffect, useRef } from 'react';
import { OrbitControls } from '@react-three/drei'
import { useFrame, Canvas } from '@react-three/fiber';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader';
import { VRM, VRMUtils, VRMLoaderPlugin } from '@pixiv/three-vrm';
import { VRMAnimationLoaderPlugin, VRMAnimation, createVRMAnimationClip } from "@pixiv/three-vrm-animation";

interface ModelProps {
	url: string
	url_anim: string
}

const VRMModel: React.FC<ModelProps> = ({ url, url_anim }) => {

	const [vrm, setVrm] = useState<VRM | null>(null);
	const mixerRef = useRef<THREE.AnimationMixer | null>(null);

	useEffect(() => {
		const loader = new GLTFLoader();
		loader.register((parser) => new VRMLoaderPlugin(parser));
		loader.register((parser) => new VRMAnimationLoaderPlugin(parser));
		loader.load(url, (gltf) => {
			const vrmModel = gltf.userData.vrm as VRM;
			VRMUtils.removeUnnecessaryJoints(vrmModel.scene);
			setVrm(vrmModel);
			const mixer = new THREE.AnimationMixer(vrmModel.scene);
			mixerRef.current = mixer;
			loader.load(url_anim, (animGltf) => {
				const vrmAnimations = animGltf.userData.vrmAnimations as VRMAnimation[];
				if (vrmAnimations && vrmAnimations.length > 0) {
					const clip = createVRMAnimationClip(vrmAnimations[0], vrmModel);
					mixer.clipAction(clip).play();
				}
			});
		});
	}, [url, url_anim]);

	useFrame((state, delta) => {
		if (mixerRef.current) mixerRef.current.update(delta);
		if (vrm) vrm.update(delta);
	});

	return vrm ? <primitive object={vrm.scene} /> : null;
};

export const VRMModelCanvas = () => {
	return (
		<div style={{ height: '100vh', width: '100vw' }}>

		<Canvas
		shadows
		gl={{
			//toneMapping: THREE.ACESFilmicToneMapping,
			//toneMapping: THREE.ReinhardToneMapping,
			toneMapping: THREE.NeutralToneMapping,
				toneMappingExposure: 1.5,
				alpha: true,
				powerPreference: "high-performance",
				antialias: true,
				//stencil: false,
				//depth: false
		}}
		camera={{ position: [1, 1, 1] }}>

		<directionalLight 
		color="white" 
		castShadow 
		position={[0, 10, 0]} 
		intensity={1.5} 
		shadow-mapSize={[1024, 1024]}/>

		<OrbitControls />
		<ambientLight intensity={1} />
		<pointLight position={[10, 10, 10]} />

		<VRMModel url="./models/default.vrm" url_anim="./models/default.vrma" />

		</Canvas>
		</div>
	)
}
export default VRMModelCanvas;
```

重要な箇所は以下のポイントです。

```html
<VRMModel url="./models/default.vrm" url_anim="./models/default.vrma" />
```

ファイルはこの場所から読み込みますので、ダウンロードする場合は[.vrm](https://vroid.pixiv.help/hc/ja/articles/31627266179865-%E3%82%B5%E3%83%B3%E3%83%97%E3%83%AB%E3%83%A2%E3%83%87%E3%83%AB%E3%81%AE%E5%88%A9%E7%94%A8%E6%96%B9%E6%B3%95%E3%81%AB%E3%81%A4%E3%81%84%E3%81%A6%E7%9F%A5%E3%82%8A%E3%81%9F%E3%81%84), [.vrma](https://booth.pm/ja/items/5512385)をここに置きます。

> src/App.tsx

```ts
import React from 'react'
import VRMModelCanvas from './pages/vrm_model'

const App = () => {
	return (
	<VRMModelCanvas/>
	)
}

export default App;
```

> package.json

```json
{
  "name": "vrm-model",
  "version": "0.1.0",
  "private": true,
  "dependencies": {
    "@pixiv/three-vrm": "^3.0.0",
    "@pixiv/three-vrm-animation": "^3.0.0",
    "@react-three/drei": "^9.109.2",
    "@react-three/fiber": "^8.16.8",
    "@react-three/postprocessing": "^2.16.2",
    "@testing-library/jest-dom": "^5.17.0",
    "@testing-library/react": "^13.4.0",
    "@testing-library/user-event": "^13.5.0",
    "@types/jest": "^27.5.2",
    "@types/node": "^16.18.104",
    "@types/react": "^18.3.3",
    "@types/react-dom": "^18.3.0",
    "@types/three": "^0.167.1",
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "react-scripts": "5.0.1",
    "three": "^0.167.1",
    "three-stdlib": "^2.30.5",
    "typescript": "^4.9.5",
    "web-vitals": "^2.1.4"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
  "eslintConfig": {
    "extends": [
      "react-app",
      "react-app/jest"
    ]
  },
  "browserslist": {
    "production": [
      ">0.2%",
      "not dead",
      "not op_mini all"
    ],
    "development": [
      "last 1 chrome version",
      "last 1 firefox version",
      "last 1 safari version"
    ]
  }
}
```

> tsconfig.json

```json
{
  "compilerOptions": {
    "target": "es5",
    "lib": [
      "dom",
      "dom.iterable",
      "esnext"
    ],
    "allowJs": true,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noFallthroughCasesInSwitch": true,
    "module": "esnext",
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  },
  "include": [
    "src"
  ]
}
```

