<template>
    <el-container>
        <el-header>添加物品类型</el-header>
        <el-main>
            <el-form :label-position="labelPosition" label-width="auto" style="max-width: 600px">
                <el-form-item label="排布" :label-position="itemLabelPosition">
                    <el-radio-group v-model="itemLabelPosition" aria-label="item label position">
                        <el-radio-button value="">Empty</el-radio-button>
                        <el-radio-button value="left">Left</el-radio-button>
                        <el-radio-button value="right">Right</el-radio-button>
                        <el-radio-button value="top">Top</el-radio-button>
                    </el-radio-group>
                </el-form-item>
                <el-form-item label="名称" :label-position="itemLabelPosition">
                    <el-input v-model="name" />
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="add_res_type(name)">添加</el-button>
                </el-form-item>
            </el-form>
        </el-main>
    </el-container>
</template>


<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage, type FormItemProps, type FormProps } from 'element-plus'


const labelPosition = ref<FormProps['labelPosition']>('right')
const itemLabelPosition = ref<FormItemProps['labelPosition']>('right')
const name = ref<string>("")

const add_res_type = (name: string) => {
    invoke("add_res_type", { name: name })
        .then((res) => {
            const msg = res as string
            ElMessage.success({ message: msg })
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
        })
}

</script>