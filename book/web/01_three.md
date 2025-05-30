# three.js

reactの`tsx`で書きます。

```sh
$ npx create-react-app galaxy --template typescript
```

これで準備はできましたが、設定ファイルを見てみます。

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

`es5`, `react-jsx`を使っているようです。

buildなどはwebpackではなく`react-scripts`ですね。

> package.json

```json
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  }
```

> src/App.tsx

```ts
import React from 'react';
import './App.css';
import { ThreeFiberGalaxy } from './pages/galaxy';

function App() {
  return <ThreeFiberGalaxy />
}

export default App;
```

> src/pages/galaxy.tsx

```ts
// https://gist.github.com/artokun/fb7f0c68a01ba5d9813abb3ccce254c4

import * as THREE from 'three'
import { Points, useGLTF } from '@react-three/drei'
import { GLTF } from 'three-stdlib'
import { useFrame, Canvas } from '@react-three/fiber'
import { useMemo, useRef } from 'react'
import { EffectComposer, SelectiveBloom } from '@react-three/postprocessing'

type GLTFResult = GLTF & {
  nodes: {
    Object_2: THREE.Mesh
  }
  materials: {
    ['Scene_-_Root']: THREE.PointsMaterial
  }
}

export function Galaxy(props: JSX.IntrinsicElements['group']) {
  const ref = useRef<THREE.Group>(null!)
  const galaxyCenterLightRef = useRef<THREE.PointLight>(null!)
  const { nodes } = useGLTF('./models/galaxy.glb') as GLTFResult
  const [positions, colors] = useMemo(() => {
    nodes.Object_2.geometry.center()
    const positions = new Float32Array(
      nodes.Object_2.geometry.attributes.position.array.buffer
    )
    const colors = new Float32Array(positions.length)

    const getDistanceToCenter = (x: number, y: number, z: number) =>
      Math.sqrt(x * x + y * y + z * z)

    // make colors closer to 0,0,0 be more reddish and colors further away be more blueish
    const color = new THREE.Color()
    for (let i = 0; i < positions.length; i += 3) {
      const x = positions[i]
      const y = positions[i + 1]
      const z = positions[i + 2]
      const distanceToCenter = getDistanceToCenter(x, y, z)
      const normalizedDistanceToCenter = distanceToCenter / 100

      // make colors closer to 0,0,0 be more reddish and colors further away be more blueish (do not use hsl)
      // color.setHSL(
      //   (0.15 * (0.21 + Math.cos(distanceToCenter * 0.02))) / 2,
      //   0.75,
      //   0.6
      // )
      color.setRGB(
        Math.cos(normalizedDistanceToCenter),
        THREE.MathUtils.randFloat(0, 0.8),
        Math.sin(normalizedDistanceToCenter)
      )
      color.toArray(colors, i)
    }

    return [positions, colors]
  }, [nodes])
  //const starTexture = useLoader(THREE.TextureLoader, '/star.png')

  // slowly rotate the galaxy
  useFrame(({ clock }) => {
    ref.current.rotation.z = clock.getElapsedTime() / 5
    // zoom in and out
    // ref.current.scale.setScalar(Math.sin(clock.getElapsedTime() / 2) + 1.5)
  })

  //  make particles glow

  return (
    <group {...props} dispose={null} ref={ref}>
      <pointLight
        position={[0, 0, 0]}
        ref={galaxyCenterLightRef}
        intensity={0.5}
      />
      <Points scale={0.05} positions={positions} colors={colors}>
        <pointsMaterial
          //map={starTexture}
          transparent
          depthWrite={false}
          vertexColors
          opacity={0.4}
          depthTest
          size={0.01}
        />
      </Points>
      <EffectComposer autoClear={false}>
        <SelectiveBloom
          intensity={2}
          luminanceThreshold={0.001}
          luminanceSmoothing={0.225}
          lights={[galaxyCenterLightRef]}
        />
      </EffectComposer>
    </group>
  )
}

useGLTF.preload('./models/galaxy.glb')
```

`App.tsx`で読み込むため以下を追記します。

> src/pages/galaxy.tsx

```ts
export const ThreeFiberGalaxy = () => {
  return (
    <Canvas>
      <ambientLight />
      <pointLight position={[10, 10, 10]} />
      <Galaxy position={[0, 0, 0]} />
    </Canvas>
  )
}
```

`.glb`は[こちら](https://sketchfab.com/3d-models/need-some-space-d6521362b37b48e3a82bce4911409303)からダウンロードして、`public/models/galaxy.glb`に置いてください。

```sh
$ npm run start
```

controlを追加します。

```sh
import { Points, useGLTF, OrbitControls } from '@react-three/drei'

export const ThreeFiberGalaxy = () => {
  return (
    <Canvas>
      <OrbitControls />
      <ambientLight />
      <pointLight position={[10, 10, 10]} />
      <Galaxy position={[0, 0, 0]} />
    </Canvas>
  )
}
```

