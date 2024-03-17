import { writable } from "svelte/store";
import type { Writable } from "svelte/store";
import type { Pod } from "../lib/types";

interface SSEevent {
    data: string;
}

const podStore: Writable<Pod[]> = writable([]);

function updatePod(event: SSEevent) { 
        const eventData: Pod = JSON.parse(event.data);
    
        const podName = eventData.metadata?.name;
        const volumes = eventData.spec?.volumes;
        const phase = eventData.status?.phase;
        const deletionGracePeriod = eventData.metadata?.deletionGracePeriodSeconds;
    
        let rowClass = "";
        if (phase === "Running") {
            rowClass = "green";
        } else if (phase === "Pending") {
            rowClass = "blue";
        } else if (phase === "Succeeded" || phase === "Failed") {
            rowClass = "delete";
        } else {
            rowClass = "amber";
        }
    
        if (deletionGracePeriod) {
            rowClass = "amber";
        }
    
        // Update the store
        podStore.update(pods => {
            // Check if the pod already exists in the store
            const existingPodIndex = pods.findIndex(pod => pod.metadata?.name === podName);
            if (existingPodIndex !== -1) {
                if (rowClass === "delete") {
                    // Remove the pod from the store
                    pods.splice(existingPodIndex, 1);
                } else {
                    // Update the existing pod in the store
                    pods[existingPodIndex].spec.volumes = volumes;
                    pods[existingPodIndex].rowClass = rowClass;
                }
            } else if (rowClass !== "delete") {
                // Add a new pod to the store
                eventData.rowClass = rowClass;
                pods.push(eventData);
            }
            return pods;
        });
};    

export { updatePod , podStore };
