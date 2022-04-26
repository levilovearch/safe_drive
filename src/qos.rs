pub mod policy;

use crate::rcl;
use num_traits::FromPrimitive;
use policy::*;
use std::time::Duration;

pub struct Profile {
    history: HistoryPolicy,

    /// Size of the message queue.
    depth: usize,

    /// Reliabiilty QoS policy setting.
    reliability: ReliabilityPolicy,

    /// Durability QoS policy setting.
    durability: DurabilityPolicy,

    /// The period at which messages are expected to be sent/received.
    /// RMW_DURATION_UNSPEFICIED will use the RMW implementation's default value,
    /// which may or may not be infinite.
    /// RMW_DURATION_INFINITE explicitly states that messages never miss a deadline expectation.
    deadline: Duration,

    /// The age at which messages are considered expired and no longer valid.
    /// RMW_DURATION_UNSPEFICIED will use the RMW implementation's default value,
    /// which may or may not be infinite.
    /// RMW_DURATION_INFINITE explicitly states that messages do not expire.
    lifespan: Duration,

    /// Liveliness QoS policy setting.
    liveliness: LivelinessPolicy,

    /// The time within which the RMW node or publisher must show that it is alive.
    /// RMW_DURATION_UNSPEFICIED will use the RMW implementation's default value,
    /// which may or may not be infinite.
    /// RMW_DURATION_INFINITE explicitly states that liveliness is not enforced.
    liveliness_lease_duration: Duration,

    /// If true, any ROS specific namespacing conventions will be circumvented.
    /// In the case of DDS and topics, for example, this means the typical
    /// ROS specific prefix of `rt` would not be applied as described here:
    /// <http://design.ros2.org/articles/topic_and_service_names.html#ros-specific-namespace-prefix>.
    /// This might be useful when trying to directly connect a native DDS topic
    /// with a ROS 2 topic.
    avoid_ros_namespace_conventions: bool,
}

impl Default for Profile {
    /// Default QoS class
    ///    - History: Keep last,
    ///    - Depth: 10,
    ///    - Reliability: Reliable,
    ///    - Durability: Volatile,
    ///    - Deadline: Default,
    ///    - Lifespan: Default,
    ///    - Liveliness: System default,
    ///    - Liveliness lease duration: Default,
    ///    - Avoid ros namespace conventions: false
    fn default() -> Self {
        Self {
            history: HistoryPolicy::KeepLast,
            depth: 10,
            reliability: ReliabilityPolicy::Reliable,
            durability: DurabilityPolicy::Volatile,
            ..Self::common()
        }
    }
}

impl Profile {
    const fn common() -> Self {
        Self {
            history: HistoryPolicy::SystemDefault,
            depth: rcl::RMW_QOS_POLICY_DEPTH_SYSTEM_DEFAULT as usize,
            reliability: ReliabilityPolicy::SystemDefault,
            durability: DurabilityPolicy::SystemDefault,
            deadline: Duration::ZERO,
            lifespan: Duration::ZERO,
            liveliness: LivelinessPolicy::SystemDefault,
            liveliness_lease_duration: Duration::ZERO,
            avoid_ros_namespace_conventions: false,
        }
    }
}

impl From<rcl::rmw_qos_profile_t> for Profile {
    fn from(qos: rcl::rmw_qos_profile_t) -> Self {
        Self {
            history: FromPrimitive::from_u32(qos.history).unwrap_or(HistoryPolicy::Unknown),
            depth: qos.depth as usize,
            reliability: FromPrimitive::from_u32(qos.reliability)
                .unwrap_or(ReliabilityPolicy::Unknown),
            durability: FromPrimitive::from_u32(qos.durability)
                .unwrap_or(DurabilityPolicy::Unknown),
            liveliness: FromPrimitive::from_u32(qos.liveliness)
                .unwrap_or(LivelinessPolicy::Unknown),
            deadline: Duration::new(qos.deadline.sec, qos.deadline.nsec as u32),
            lifespan: Duration::new(qos.lifespan.sec, qos.lifespan.nsec as u32),
            liveliness_lease_duration: Duration::new(
                qos.liveliness_lease_duration.sec,
                qos.liveliness_lease_duration.nsec as u32,
            ),
            avoid_ros_namespace_conventions: qos.avoid_ros_namespace_conventions,
        }
    }
}