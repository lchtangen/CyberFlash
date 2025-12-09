<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

interface GithubArtifact {
  id: number;
  name: string;
  size_in_bytes: number;
  created_at: string;
  download_url: string;
}

interface GithubWorkflowRun {
  id: number;
  name: string;
  status: string;
  conclusion: string | null;
  created_at: string;
}

const token = ref('');
const owner = ref('');
const repo = ref('');
const artifacts = ref<GithubArtifact[]>([]);
const runs = ref<GithubWorkflowRun[]>([]);
const isLoading = ref(false);
const error = ref('');

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

async function fetchBuilds() {
  if (!token.value || !owner.value || !repo.value) return;
  
  isLoading.value = true;
  error.value = '';
  artifacts.value = [];
  runs.value = [];

  try {
    // Parallel fetch
    const [artifactsRes, runsRes] = await Promise.all([
      invoke<GithubArtifact[]>('list_github_artifacts', { token: token.value, owner: owner.value, repo: repo.value }),
      invoke<GithubWorkflowRun[]>('list_recent_runs', { token: token.value, owner: owner.value, repo: repo.value })
    ]);
    
    artifacts.value = artifactsRes;
    runs.value = runsRes;
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
  }
}

function download(url: string) {
  // In a real app, we'd use the download manager
  window.open(url, '_blank');
}
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">cloud_sync</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Cloud Build Sync</h2>
        <p class="text-sm text-white/60">Fetch artifacts from GitHub Actions.</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
        <GlassInput v-model="owner" placeholder="Owner (e.g. LineageOS)" />
        <GlassInput v-model="repo" placeholder="Repo (e.g. android_device_oneplus_guacamole)" />
        <GlassInput v-model="token" type="password" placeholder="GitHub Token (ghp_...)" />
      </div>
      
      <VisionButton @click="fetchBuilds" :disabled="isLoading || !token" class="w-full">
        <span v-if="isLoading" class="animate-spin material-symbols-rounded">sync</span>
        <span v-else>Fetch Builds</span>
      </VisionButton>

      <div v-if="error" class="p-3 bg-error/10 border border-error/20 rounded-lg text-error text-sm">
        {{ error }}
      </div>

      <div v-if="runs.length > 0" class="mt-6">
        <h3 class="text-sm font-bold text-white mb-3 uppercase tracking-wider">Recent Workflow Runs</h3>
        <div class="space-y-2">
          <div v-for="run in runs" :key="run.id" class="flex items-center justify-between p-3 rounded-lg bg-white/5 border border-white/10">
            <div class="flex items-center gap-3">
              <div class="w-2 h-2 rounded-full" :class="{
                'bg-success': run.conclusion === 'success',
                'bg-error': run.conclusion === 'failure',
                'bg-warning': run.status === 'in_progress'
              }"></div>
              <div>
                <div class="font-medium text-white">{{ run.name }}</div>
                <div class="text-xs text-white/60">{{ new Date(run.created_at).toLocaleString() }}</div>
              </div>
            </div>
            <div class="text-xs px-2 py-1 rounded bg-black/20 text-white/60 capitalize">{{ run.status }}</div>
          </div>
        </div>
      </div>

      <div v-if="artifacts.length > 0" class="mt-6">
        <h3 class="text-sm font-bold text-white mb-3 uppercase tracking-wider">Available Artifacts</h3>
        <div class="space-y-2">
          <div v-for="artifact in artifacts" :key="artifact.id" class="flex items-center justify-between p-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 transition-colors">
            <div class="flex items-center gap-3">
              <span class="material-symbols-rounded text-primary">folder_zip</span>
              <div>
                <div class="font-medium text-white">{{ artifact.name }}</div>
                <div class="text-xs text-white/60">{{ formatBytes(artifact.size_in_bytes) }}</div>
              </div>
            </div>
            <VisionButton size="sm" icon="download" @click="download(artifact.download_url)">Download</VisionButton>
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
