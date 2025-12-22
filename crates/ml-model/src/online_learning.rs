/// 在线学习模块 - 支持模型持续更新
use std::collections::VecDeque;
use chrono::{DateTime, Utc};

/// 训练样本
#[derive(Debug, Clone)]
pub struct TrainingSample {
    pub features: Vec<f64>,
    pub label: f64,
    pub timestamp: DateTime<Utc>,
}

/// 在线学习缓冲区
pub struct OnlineLearningBuffer {
    buffer: VecDeque<TrainingSample>,
    max_size: usize,
    min_batch_size: usize,
}

impl OnlineLearningBuffer {
    pub fn new(max_size: usize, min_batch_size: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
            min_batch_size,
        }
    }

    /// 添加训练样本
    pub fn add_sample(&mut self, sample: TrainingSample) {
        if self.buffer.len() >= self.max_size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(sample);
    }

    /// 检查是否可以进行批量训练
    pub fn is_ready_for_training(&self) -> bool {
        self.buffer.len() >= self.min_batch_size
    }

    /// 获取训练批次
    pub fn get_training_batch(&self) -> Vec<TrainingSample> {
        self.buffer.iter().cloned().collect()
    }

    /// 清空缓冲区
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// 获取缓冲区大小
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_online_learning_buffer() {
        let mut buffer = OnlineLearningBuffer::new(100, 10);

        for i in 0..15 {
            buffer.add_sample(TrainingSample {
                features: vec![i as f64; 50],
                label: i as f64,
                timestamp: Utc::now(),
            });
        }

        assert!(buffer.is_ready_for_training());
        assert_eq!(buffer.len(), 15);

        let batch = buffer.get_training_batch();
        assert_eq!(batch.len(), 15);
    }
}
