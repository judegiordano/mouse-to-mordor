/// <reference types="vite-plugin-svgr/client" />
import { useEffect, useLayoutEffect, useRef, useState } from 'react'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

import { Progress } from './types'

import mordorPathSvg from './assets/mordorPath.svg?raw'
// import Map from '/clean-map.webp'

export default function App() {
	const [mousePosition, setMousePosition] = useState<Progress | null>(null)
	const unlistenRef = useRef<UnlistenFn | null>(null)
	const [streaming, setStreaming] = useState(false)

	async function startStream() {
		if (streaming) return
		try {
			await invoke('start_stream')
			const unlistenMousePosition = await listen<Progress>('distance-traveled', (event) => {
				setMousePosition(event.payload)
			})
			unlistenRef.current = unlistenMousePosition
			setStreaming(true)
		} catch (err) {
			console.error('start_stream failed', err)
		}
	}

	async function stopStream() {
		if (!streaming) return
		try {
			await invoke('stop_stream')
		} catch (err) {
			console.error('stop_stream failed', err)
		}
		if (unlistenRef.current) {
			try {
				unlistenRef.current()
			} catch { }
			unlistenRef.current = null
		}
		setStreaming(false)
	}

	useEffect(() => {
		startStream()
		return () => {
			stopStream()
		}
	}, [])

	// function Progress({ landmark }: { landmark: [Landmark, number] }) {
	// 	const [name, distance] = landmark
	// 	let c = ''
	// 	let rotate = ''
	// 	switch (name) {
	// 		case 'THE_SHIRE_TO_BREE':
	// 			c = 'pt-44 pl-70'
	// 			rotate = 'rotate-50'
	// 			break
	// 		case 'BREE_TO_RIVENDELL':
	// 			c = 'pt-62 pl-80'
	// 			rotate = 'rotate-60'
	// 			break
	// 		default:
	// 			break
	// 	}
	// 	return (
	// 		<div className={`absolute ${c}`}>
	// 			<p className="text-black font-bold select-none">{name.split('_').join(' ')}</p>
	// 			<progress
	// 				className={`progress progress-neutral w-20 ${rotate}`}
	// 				value={mousePosition?.distance_traveled.total_miles_traveled}
	// 				max={distance}
	// 			/>
	// 		</div>
	// 	)
	// 	return <></>
	// }

	return (
		<div
			className="mx-auto flex flex-col h-screen gap-6 bg-amber-100"
		// style={{
		// 	// backgroundImage: `url(${Map})`,
		// 	backgroundSize: 'contain',
		// 	backgroundRepeat: 'no-repeat',
		// 	backgroundPosition: 'center',
		// }}
		>

			{/* <div className="relative w-full h-full">
				<Map className="absolute inset-0 w-full h-full object-contain" />
				<Path className="absolute inset-0 w-full h-full object-contain" />
			</div> */}

			<div className="relative w-screen h-screen overflow-hidden">
				<AnimatedMordorPath
					svgContent={mordorPathSvg}
					progress={mousePosition?.distance_traveled.total_inches_traveled || 0}
					maxDistance={(mousePosition?.landmarks.find(([name]) => name === 'TOTAL_WALKING_DISTANCE')?.[1] || 1779) * 63360} // maybe calc percentage on backend to pass in directly
				/>
			</div>

			{/* {
				mousePosition?.landmarks.map((landmark) => (
					<Progress landmark={landmark as unknown as [Landmark, number]} />
					// <div>
					// 	<p className="text-black font-bold">
					// 		{landmark[0].split('_').join(' ')}
					// 	</p>
					// 	<progress
					// 		key={key}
					// 		className="progress progress-neutral w-56"
					// 		value={mousePosition?.distance_traveled.total_miles_traveled}
					// 		max={landmark[1]}>
					// 	</progress>
					// </div>
				))
			} */}

			<div className="absolute bottom-0 text-black font-bold pb-10 pl-20 select-none">
				<p>
					{mousePosition?.distance_traveled.total_inches_traveled} inches
				</p>
				<p>
					{mousePosition?.distance_traveled.total_feet_traveled} feet
				</p>
				<p>
					{mousePosition?.distance_traveled.total_miles_traveled} mile{mousePosition?.distance_traveled.total_miles_traveled === 1 ? '' : 's'}
				</p>
			</div>
			{/* <div className="pt-[220px]">
				<DiagonalProgress progress={12} pathD="M 10 10 L 290 90" />
			</div> */}
			{/* <DiagonalProgress progress={12} /> */}
		</div>

		// <div className="p-6 max-w-7xl mx-auto flex flex-col gap-6">
		// 	<div>
		// 		<progress
		// 			className="progress w-56"
		// 			value={mousePosition?.distance_traveled.total_miles_traveled}
		// 			max={mousePosition?.landmarks.total_walking_distance}>
		// 		</progress>
		// 	</div>
		// 	<div>
		// 		{mousePosition?.distance_traveled.total_inches_traveled} inches
		// 	</div>
		// 	<div>
		// 		{mousePosition?.distance_traveled.total_feet_traveled} feet
		// 	</div>
		// 	<div>
		// 		{mousePosition?.distance_traveled.total_miles_traveled} miles
		// 	</div>
		// </div>
	)
}

function AnimatedMordorPath({
	svgContent,
	progress = 0,
	maxDistance = 1779,
}: {
	svgContent: string
	progress: number
	maxDistance: number
}) {
	const containerRef = useRef<HTMLDivElement | null>(null)
	const pathLengthRef = useRef<number>(0)
	const maskPathRef = useRef<SVGPathElement | null>(null)

	useEffect(() => {
		if (!containerRef.current || maskPathRef.current) return

		containerRef.current.innerHTML = svgContent

		const maskPath = containerRef.current.querySelector('#pathMask') as SVGPathElement
		if (maskPath) {
			maskPathRef.current = maskPath
			pathLengthRef.current = maskPath.getTotalLength()
			console.log('Initialized - Path length:', pathLengthRef.current)
		}
	}, [svgContent])

	useLayoutEffect(() => {
		if (!maskPathRef.current || pathLengthRef.current === 0) return

		// update these values locally to see faster progress.
		// progress = progress * 1000
		const progressPercent = Math.max(0, Math.min(progress / maxDistance, 1))
		const offset = pathLengthRef.current * (1 - progressPercent)

		maskPathRef.current.style.strokeDasharray = `${pathLengthRef.current}`
		maskPathRef.current.style.strokeDashoffset = `${offset}`

		console.log('Updated:', {
			progress,
			maxDistance,
			progressPercent: (progressPercent * 100).toFixed(1) + '%',
			offset: offset.toFixed(1)
		})
	}, [progress, maxDistance])

	return (
		<div
			ref={containerRef}
			className="absolute inset-0 w-full h-full"
		/>
	)
}

export function DiagonalProgress({
	progress = 0,
	strokeWidth = 8,
	color = '#22C55E',
	trailColor = '#E5E7EB',
	className = '',
	// bottom-left → top-right
	pathD = 'M 10 90 L 290 10'
}) {
	const pathRef = useRef<SVGPathElement | null>(null)
	const [len, setLen] = useState(0)

	useEffect(() => {
		if (!pathRef.current) return
		const total = pathRef.current.getTotalLength()
		setLen(total)
	}, [])

	const pct = Math.max(0, Math.min(progress, 100))
	const offset = len - (len * pct) / 100

	return (
		<svg
			width="100%"
			height="120"
			viewBox="0 0 300 100"
			className={className}
		>
			<path
				d={pathD}
				stroke={trailColor}
				strokeWidth={strokeWidth}
				strokeLinecap="round"
			/>
			<path
				ref={pathRef}
				d={pathD}
				stroke={color}
				strokeWidth={strokeWidth}
				strokeLinecap="round"
				strokeDasharray={len}
				strokeDashoffset={offset}
				style={{ transition: 'stroke-dashoffset 0.5s ease-out' }}
			/>
		</svg>
	)
}
