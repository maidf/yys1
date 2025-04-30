<template>
    <el-container>
        <el-header>活动</el-header>
        <el-main v-if="activity">
            <el-table :data="activity" style="width: 100%">
                <el-table-column fixed prop="name" label="Name" width="150" />
                <el-table-column prop="num" label="Num" width="120" />
                <el-table-column prop="consume" label="Consume" width="120" />
                <el-table-column fixed="right" label="Operations" min-width="120">
                    <template #default="{ row }">
                        <el-button type="primary" size="small" @click="">
                            Detail
                        </el-button>
                        <el-button type="danger" size="small" @click="rm_activity(row.id)">Delete</el-button>
                    </template>
                </el-table-column>
            </el-table>
        </el-main>
        <el-main v-else></el-main>
    </el-container>
</template>


<script setup lang="ts">
import { onMounted, ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage } from 'element-plus'
import { Activity } from "../stores/type"

const activity = ref<Activity[]>([])
const get_activity = () => {
    console.log("invoke -> get activity")
    invoke("get_activity")
        .then((res) => {
            activity.value = res as Activity[]
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
            activity.value = []
        })
}

const rm_activity = (id: string) => {
    console.log("invoke -> rm activity")
    invoke("rm_activity", { id: id })
        .then((res) => {
            activity.value = activity.value.filter((item) => item.id !== id)
            const msg = res as string
            ElMessage.success({ message: msg })
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
        })
}

onMounted(() => {
    get_activity()
    console.log("mount -> get activity")
})
</script>