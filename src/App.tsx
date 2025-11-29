import { useEffect, useRef, useState } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Progress } from "./types";

import Map from '/map.svg'

export default function App() {
	const [mousePosition, setMousePosition] = useState<Progress | null>(null);
	const unlistenRef = useRef<UnlistenFn | null>(null);
	const [streaming, setStreaming] = useState(false);

	async function startStream() {
		if (streaming) return;
		try {
			await invoke("start_stream");
			const unlistenMousePosition = await listen<Progress>("distance-traveled", (event) => {
				setMousePosition(event.payload);
			});
			unlistenRef.current = unlistenMousePosition;
			setStreaming(true);
		} catch (err) {
			console.error("start_stream failed", err);
		}
	}

	async function stopStream() {
		if (!streaming) return;
		try {
			await invoke("stop_stream");
		} catch (err) {
			console.error("stop_stream failed", err);
		}
		if (unlistenRef.current) {
			try {
				unlistenRef.current();
			} catch { }
			unlistenRef.current = null;
		}
		setStreaming(false);
	}

	useEffect(() => {
		startStream()
		return () => {
			stopStream()
		}
	}, [])

	return (
		<div
			className="p-6 mx-auto flex flex-col h-screen gap-6 bg-amber-100"
			style={{
				backgroundImage: `url(${Map})`,
				backgroundSize: "contain",
				backgroundRepeat: "no-repeat",
				backgroundPosition: "center",
			}}
		>
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
	);
}
