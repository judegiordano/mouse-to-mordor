import { useEffect, useRef, useState } from 'react'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

import { Landmark, Progress } from './types'

// import Map from '/map.svg'
import Map from '/clean-map.webp'

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

	function Progress({ landmark }: { landmark: [Landmark, number] }) {
		const [name, distance] = landmark
		let c = ''
		let rotate = ''
		switch (name) {
			case 'THE_SHIRE_TO_BREE':
				c = 'pt-44 pl-70'
				rotate = 'rotate-50'
				break
			case 'BREE_TO_RIVENDELL':
				c = 'pt-62 pl-80'
				rotate = 'rotate-60'
				break
			default:
				break
		}
		return (
			<div className={`absolute ${c}`}>
				{/* rotate-310 */}
				<p className="text-black font-bold select-none">{name.split('_').join(' ')}</p>
				{/* <button onClick={() => alert()}>
					<ExclamationCircleOutlined style={{ color: 'black' }} />
				</button> */}

				{/* <Tooltip title="search">
					<Button type="secondary" shape="circle" icon={<ExclamationCircleOutlined style={{ color: 'black' }} />} />
				</Tooltip> */}

				<progress
					className={`progress progress-neutral w-20 ${rotate}`}
					value={mousePosition?.distance_traveled.total_miles_traveled}
					max={distance}
				/>
			</div>
		)
		return <></>
	}

	return (
		<div
			className="mx-auto flex flex-col h-screen gap-6 bg-amber-100"
			style={{
				backgroundImage: `url(${Map})`,
				backgroundSize: 'contain',
				backgroundRepeat: 'no-repeat',
				backgroundPosition: 'center',
			}}
		>
			{
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
			}
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
